"""Define trainable machine learning models."""

# %% IMPORTS

import abc
import asyncio
import json
import os
import typing as T
from datetime import datetime, timezone
from typing import Any, Dict, Optional

import pandas as pd
import pydantic as pdt
from agent_framework import ChatMessage, ChatResponse
from agent_framework.openai import OpenAIChatClient
from pydantic import Field, PrivateAttr

from autogen_team.core import schemas

# %% TYPES

# Model params
ParamKey = str
ParamValue = T.Any
Params = dict[ParamKey, ParamValue]


# %% MODELS


class Model(
    abc.ABC, pdt.BaseModel, strict=True, frozen=False, extra="forbid", arbitrary_types_allowed=True
):
    """Base class for a project model.

    Use a model to adapt AI/ML frameworks.
    e.g., to swap easily one model with another.
    """

    KIND: str

    def get_params(self, deep: bool = True) -> Params:
        """Get the model params.

        Args:
            deep (bool, optional): ignored.

        Returns:
            Params: internal model parameters.
        """
        params: Params = {}
        for key, value in self.__dict__.items():
            if not key.startswith("_") and not key.isupper():
                params[key] = value
        return params

    def set_params(self, **params: ParamValue) -> T.Self:
        """Set the model params in place.

        Returns:
            T.Self: instance of the model.
        """
        for key, value in params.items():
            setattr(self, key, value)
        return self

    @abc.abstractmethod
    def load_context(self, model_config: Dict[str, Any]) -> None:
        """
        Load the model from the specified artifacts directory.
        """

    @abc.abstractmethod
    def fit(self, inputs: schemas.Inputs, targets: schemas.Targets) -> T.Self:
        """Fit the model on the given inputs and targets.

        Args:
            inputs (schemas.Inputs): model training inputs.
            targets (schemas.Targets): model training targets.

        Returns:
            T.Self: instance of the model.
        """

    @abc.abstractmethod
    def predict(self, inputs: schemas.Inputs) -> schemas.Outputs:
        """Generate outputs with the model for the given inputs.

        Args:
            inputs (schemas.Inputs): model prediction inputs.

        Returns:
            schemas.Outputs: model prediction outputs.
        """

    def explain_model(self) -> schemas.FeatureImportances:
        """Explain the internal model structure.

        Raises:
            NotImplementedError: method not implemented.

        Returns:
            schemas.FeatureImportances: feature importances.
        """
        raise NotImplementedError()

    def explain_samples(self, inputs: schemas.Inputs) -> schemas.SHAPValues:
        """Explain model outputs on input samples.

        Raises:
            NotImplementedError: method not implemented.

        Returns:
            schemas.SHAPValues: SHAP values.
        """
        raise NotImplementedError()

    def get_internal_model(self) -> T.Any:
        """Return the internal model in the object.

        Raises:
            NotImplementedError: method not implemented.

        Returns:
            T.Any: any internal model (either empty or fitted).
        """
        raise NotImplementedError()


class BaselineAutogenModel(Model):
    """Simple baseline model based on autogen.
    https://microsoft.github.io/autogen/stable/user-guide/core-user-guide/design-patterns/group-chat.html
    Parameters:
        max_tokens (int): maximum token of the prompt
        temperature (float): temperature for the sampling
    """

    KIND: T.Literal["BaselineAutogenModel"] = "BaselineAutogenModel"
    model_config_path: Optional[str] = Field(default=None)
    model_config_data: Optional[Dict[str, Any]] = Field(default=None)
    _model_client: Optional[OpenAIChatClient] = PrivateAttr(default=None)
    max_tokens: Optional[int] = Field(default=320000)
    temperature: Optional[float] = Field(default=0.5)

    def __init__(
        self,
        model_config_path: Optional[str] = None,
        model_config_data: Optional[Dict[str, Any]] = None,
        max_tokens: Optional[int] = 320000,
        temperature: Optional[float] = 0.5,
        **data: Any,
    ) -> None:
        super().__init__(  # type: ignore[call-arg]
            model_config_path=model_config_path,
            model_config_data=model_config_data,
            max_tokens=max_tokens,
            temperature=temperature,
            **data,
        )
        # Ensure sklearn's clone test passes by re-assigning the exact same objects
        self.model_config_path = model_config_path
        self.model_config_data = model_config_data
        self.max_tokens = max_tokens
        self.temperature = temperature

    def load_context_path(self, model_config_path: Optional[str] = None) -> None:
        """
        Load the model from the specified artifacts directory.
        """
        if self._model_client is not None:
            return

        if model_config_path is None:
            raise ValueError("No configuration file path provided.")

        if not os.path.isfile(model_config_path):
            raise FileNotFoundError(f"Configuration file '{model_config_path}' not found.")

        with open(model_config_path, "r", encoding="utf-8") as config_file:
            model_config = json.load(config_file)

        # Load the model
        self.load_context(model_config)

    def load_context(self, model_config: Dict[str, Any]) -> None:
        """
        Load the model from the specified artifacts directory.
        https://microsoft.github.io/autogen/stable/user-guide/agentchat-user-guide/migration-guide.html#assistant-agent
        https://microsoft.github.io/autogen/stable/user-guide/core-user-guide/cookbook/local-llms-ollama-litellm.html

        """
        # Store config for lazy loading/cloning
        self.model_config_data = model_config

        # Handle env var substitution for API key
        api_key = model_config["config"]["api_key"]
        if api_key == "${LITELLM_API_KEY}":
            api_key = os.getenv("LITELLM_API_KEY")
            if not api_key:
                raise ValueError("LITELLM_API_KEY environment variable is not set")

        # Load the client
        self._model_client = OpenAIChatClient(
            model_id=model_config["config"]["model"],
            api_key=api_key,
            base_url=model_config["config"]["api_base"],
        )

    def fit(self, inputs: schemas.Inputs, targets: schemas.Targets) -> "BaselineAutogenModel":
        # TBD LORA project Iñaki
        # self.load_context(model_config={})
        return self

    async def _rungroupchat(self, content: str) -> ChatResponse:
        """Executes a group chat request using the model client."""
        if not self._model_client:
            if self.model_config_data:
                self.load_context(self.model_config_data)
            else:
                raise RuntimeError("Model client is not initialized and no config available.")

        try:
            # Create the message using agent_framework ChatMessage
            message = ChatMessage(text=content, role="user")

            # Use get_response instead of create
            response: ChatResponse = await self._model_client.get_response(messages=[message])

        except Exception as e:
            # Create a dummy response for error
            # Note: ChatResponse init args might differ, putting rudimentary error handling
            # Assuming ChatResponse has messages or content?
            # agent_framework ChatResponse usually has 'messages' list of ChatMessage
            # I will create a dummy ChatResponse if possible, or re-raise
            # For now, let's just log and return a basic structure or raise
            raise e
            # response = ChatResponse(
            #     content=f"Error: {e}", call_id="", finish_reason="error", usage={}, cached=False
            # )

        return response

    def predict(self, inputs: schemas.Inputs) -> schemas.Outputs:
        """
        Predicts the output using the assistant team based on the given inputs.
        Processes each input element iteratively and appends results to the output DataFrame.
        """
        # Initialize a list to collect messages or results
        results = []

        # Iterate over each input element
        for row in inputs.itertuples(index=False):
            response: ChatResponse = asyncio.run(self._rungroupchat(str(row.input)))

            if response and response.messages:  # Check if response has messages
                # content is likely in the last message or usage?
                # agent_framework ChatResponse has 'messages' list and 'text' property.
                content_str = response.text

                results.append(
                    {
                        "response": content_str,  # Getting the response content
                        "metadata": {
                            "timestamp": datetime.now(
                                timezone.utc
                            ).isoformat(),  # Current time in ISO-8601 format
                            "model_version": "v1.0.0",
                            "terminated": response.finish_reason is not None,
                            "messages": [msg.text for msg in response.messages],
                        },
                    }
                )

        # Prepare outputs schema

        outputs = schemas.Outputs(
            pd.DataFrame(results)  # Create DataFrame from the list of dictionaries
        )
        return outputs

    def get_internal_model(self) -> OpenAIChatClient:
        if not self._model_client and self.model_config_data:
            self.load_context(self.model_config_data)

        if isinstance(self._model_client, OpenAIChatClient):
            return self._model_client
        else:
            raise ValueError("Model client is not initialized or is of incorrect type.")

    def explain_model(self) -> schemas.FeatureImportances:
        """
        Provides a text-based explanation of the model's internal structure.
        Since this model leverages the OpenAI Chat API for generating responses,
        it does not produce traditional numerical feature importances.
        """
        explanation = {
            "feature": (
                "BaselineAutogenModel utilizes the OpenAI Chat Completion client to generate responses "
                "in a group chat setting. Unlike conventional machine learning models that compute "
                "numerical feature importances, this model relies on prompt-driven generation and context "
                "management to produce outputs. As such, it does not support feature importance metrics in the usual sense."
            ),
            "importance": 1.0,
        }

        # Create DataFrame from a list of dictionaries (one row)
        explanation_df = pd.DataFrame([explanation])
        return schemas.FeatureImportances(explanation_df)

    def explain_samples(self, inputs: schemas.Inputs) -> schemas.SHAPValues:
        """
        Explains model outputs for the given input samples by leveraging the predict function.
        For each input, a textual explanation is provided along with a dummy SHAP value.
        """
        explanations = []

        # Obtain predictions for the input samples
        outputs = self.predict(inputs)
        # Assuming outputs is a DataFrame; if wrapped in an attribute, adjust accordingly.
        output_df = outputs

        # Iterate over each input and its corresponding prediction to build explanations.
        for input_row, output_row in zip(
            inputs.itertuples(index=False), output_df.itertuples(index=False)
        ):
            explanation_text = (
                f"For input '{input_row.input}', the model generated response '{output_row.response}'. "
                "This response is produced using prompt-driven generation and context management. "
                "Since traditional SHAP values are not applicable for a chat-based model, a dummy attribution of 1.0 is used."
            )
            explanations.append(
                {"sample": input_row.input, "explanation": explanation_text, "shap_value": 1.0}
            )

        explanation_df = pd.DataFrame(explanations)
        # Return the DataFrame as a SHAPValues type. Note that schemas.SHAPValues is defined as a type alias.
        return schemas.SHAPValues(explanation_df)

    def __getstate__(self) -> Dict[str, Any]:
        """Custom getstate to exclude unpicklable model client while preserving Pydantic state."""
        state = self.__dict__.copy()
        # Include internal attributes that Pydantic uses for its internal state
        for attr in ["__pydantic_fields_set__", "__pydantic_extra__", "__pydantic_private__"]:
            if hasattr(self, attr):
                val = getattr(self, attr)
                if attr == "__pydantic_private__" and val:
                    # Exclude the unpicklable client from private state
                    val = {k: v for k, v in val.items() if k != "_model_client"}
                state[attr] = val
        return state

    def __setstate__(self, state: Dict[str, Any]) -> None:
        """Custom setstate to restore the model state including Pydantic internal state."""
        # Restore internal attributes first
        for attr in ["__pydantic_fields_set__", "__pydantic_extra__", "__pydantic_private__"]:
            if attr in state:
                val = state.pop(attr)
                try:
                    setattr(self, attr, val)
                except Exception:
                    # Fallback if setattr fails for some reason
                    object.__setattr__(self, attr, val)

        # Restore the rest of the attributes
        self.__dict__.update(state)

        # Ensure _model_client is initialized to None in __pydantic_private__
        if not hasattr(self, "__pydantic_private__") or self.__pydantic_private__ is None:
            object.__setattr__(self, "__pydantic_private__", {})

        if self.__pydantic_private__ is not None:
            self.__pydantic_private__["_model_client"] = None


ModelKind = BaselineAutogenModel
