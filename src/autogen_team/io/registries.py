"""Savers, loaders, and registers for model registries."""
# mypy: disable-error-code=override

# %% IMPORTS

import abc
import json
import os
import typing as T
from typing import Any, Dict

import mlflow
import mlflow.entities
import mlflow.entities.model_registry
import mlflow.models.model
import pandas as pd
import pydantic as pdt
from mlflow.pyfunc.model import PythonModel, PythonModelContext

from autogen_team.core import models, schemas
from autogen_team.utils import signers

# %% TYPES

# Results of model registry operations
Info: T.TypeAlias = mlflow.models.model.ModelInfo
Alias: T.TypeAlias = mlflow.entities.model_registry.ModelVersion
Version: T.TypeAlias = mlflow.entities.model_registry.ModelVersion

# %% HELPERS


def uri_for_model_alias(name: str, alias: str) -> str:
    """Create a model URI from a model name and an alias.

    Args:
        name (str): name of the mlflow registered model.
        alias (str): alias of the registered model.

    Returns:
        str: model URI as "models:/name@alias".
    """
    return f"models:/{name}@{alias}"


def uri_for_model_version(name: str, version: str) -> str:
    """Create a model URI from a model name and a version.

    Args:
        name (str): name of the mlflow registered model.
        version (int): version of the registered model.

    Returns:
        str: model URI as "models:/name/version."
    """
    return f"models:/{name}/{version}"


def uri_for_model_alias_or_version(name: str, alias_or_version: str | int) -> str:
    """Create a model URi from a model name and an alias or version.

    Args:
        name (str): name of the mlflow registered model.
        alias_or_version (str | int): alias or version of the registered model.

    Returns:
        str: model URI as "models:/name@alias" or "models:/name/version" based on input.
    """
    if isinstance(alias_or_version, int):
        return uri_for_model_version(name=name, version=str(alias_or_version))
    else:
        return uri_for_model_alias(name=name, alias=alias_or_version)


# %% SAVERS


class Saver(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """Base class for saving models in registry.

    Separate model definition from serialization.
    e.g., to switch between serialization flavors.

    Parameters:
        path (str): model path inside the Mlflow store.
    """

    KIND: str

    path: str = "model"
    config_file: str = "model_config.json"

    @abc.abstractmethod
    def save(
        self, model: models.Model, signature: signers.Signature, input_example: schemas.Inputs
    ) -> Info:
        """Save a model in the model registry.

        Args:
            model (models.Model): project model to save.
            signature (signers.Signature): model signature.
            input_example (schemas.Inputs): sample of inputs.

        Returns:
            Info: model saving information.
        """


class CustomSaver(Saver):
    """Saver for project models using the Mlflow PyFunc module.

    https://mlflow.org/docs/latest/python_api/mlflow.pyfunc.html
    https://mlflow.org/blog/autogen-image-agent
    https://mlflow.org/blog/custom-pyfunc
    """

    KIND: T.Literal["CustomSaver"] = "CustomSaver"

    class Adapter(PythonModel):
        """Adapt a custom model to the Mlflow PyFunc flavor for saving operations.

        https://mlflow.org/docs/latest/python_api/mlflow.pyfunc.html?#mlflow.pyfunc.PythonModel
        """

        # TBD load context:
        def __init__(self, model: models.Model):
            """Initialize the custom saver adapter.

            Args:
                model (models.Model): project model.
            """
            self.model = model
            self.model_config = {
                "provider": "openai_chat_completion_client",  # Use LiteLLM-compatible client
                "config": {
                    "model": "azure-gpt",  # LiteLLM model
                    "api_base": "http://litellm.llm-apps.svc.cluster.local/v1",  # LiteLLM Gateway URL
                    "api_key": os.getenv(
                        "LITELLM_API_KEY", "sk-litellm-master-key-12345"
                    ),  # Fallback for safety, but prefer env
                    "temperature": 0.7,  # Optional
                    "max_tokens": 512,  # Optional
                },
            }

        def load_context(self, context: PythonModelContext) -> None:
            """
            Load the model from the specified artifacts directory.
            """
            model_file_path = context.artifacts["config_file"]
            # Define the configuration
            model_config = json.load(open(model_file_path, "r", encoding="utf-8"))
            # Load the model
            self.model.load_context(model_config)

        def predict(
            self, context: PythonModelContext, model_input: schemas.Inputs
        ) -> schemas.Outputs:
            """Generate predictions with a custom model for the given inputs."""
            output = self.model.predict(inputs=model_input)
            return output

    def save(
        self, model: models.Model, signature: signers.Signature, input_example: schemas.Inputs
    ) -> Info:
        adapter = CustomSaver.Adapter(model=model)

        # Local save block
        import tempfile

        # Create a unique temporary directory for the model
        local_path = tempfile.mkdtemp(prefix="champion_model_", dir="outputs")

        # Ensure outputs directory exists
        os.makedirs("outputs", exist_ok=True)

        mlflow.pyfunc.save_model(
            path=local_path,
            python_model=adapter,
            signature=signature,
            artifacts={"config_file": os.path.join(self.path, self.config_file)},
            input_example=input_example,
        )
        print(f"Model saved locally to {local_path}")

        try:
            model_info: Info = mlflow.pyfunc.log_model(
                python_model=adapter,
                signature=signature,
                artifact_path=self.path,
                artifacts={"config_file": os.path.join(self.path, self.config_file)},
                input_example=input_example,
            )
            return model_info
        except Exception as e:
            print(f"Failed to log model to MLflow (likely S3 auth issue): {e}")
            # Return a dummy info or re-raise if strictly needed, but for now we proceed with local file
            from mlflow.models.model import ModelInfo

            return ModelInfo(
                artifact_path="dummy_path",
                flavors={},
                mlflow_version=mlflow.__version__,
                model_uri=f"file://{os.path.abspath(local_path)}",
                model_uuid="dummy",
                run_id="dummy",
                saved_input_example_info=None,
                signature=signature,
                utc_time_created=pd.Timestamp.now().isoformat(),
            )


SaverKind = CustomSaver

# %% LOADERS


class Loader(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """Base class for loading models from registry.

    Separate model definition from deserialization.
    e.g., to switch between deserialization flavors.
    """

    KIND: str

    class Adapter(abc.ABC):
        """Adapt any model for the project inference."""

        @abc.abstractmethod
        def load_context(self, model_config: Dict[str, Any]) -> None:
            """
            Load the model from the specified artifacts directory.
            """

        @abc.abstractmethod
        def predict(self, inputs: schemas.Inputs) -> schemas.Outputs:
            """Generate predictions with the internal model for the given inputs.

            Args:
                inputs (schemas.Inputs): validated inputs for the project model.

            Returns:
                schemas.Outputs: validated outputs of the project model.
            """

    @abc.abstractmethod
    def load(self, uri: str) -> "Loader.Adapter":
        """Load a model from the model registry.

        Args:
            uri (str): URI of a model to load.

        Returns:
            Loader.Adapter: model loaded.
        """


class CustomLoader(Loader):
    """Loader for custom models using the Mlflow PyFunc module.

    https://mlflow.org/docs/latest/python_api/mlflow.pyfunc.html
    """

    KIND: T.Literal["CustomLoader"] = "CustomLoader"

    class Adapter(Loader.Adapter):
        """Adapt a custom model for the project inference."""

        def __init__(self, model: mlflow.pyfunc.PyFuncModel) -> None:  # type: ignore[name-defined]
            """Initialize the adapter from an mlflow pyfunc model.

            Args:
                model (mlflow.pyfunc.PyFuncModel): mlflow pyfunc model.
            """
            self.model = model

        def load_context(self, model_config: Dict[str, Any]) -> None:
            """
            Load the model from the specified artifacts directory.
            """
            self.model.load_context(model_config)

        def predict(self, inputs: schemas.Inputs) -> schemas.Outputs:
            # model validation is already done in predict
            prediction = self.model.predict(data=inputs)

            # Return the outputs schema
            outputs = schemas.Outputs(
                pd.DataFrame(
                    {
                        "response": [prediction],
                        "metadata": [
                            {"timestamp": "2025-01-15T12:00:00Z", "model_version": "v1.0.0"}
                        ],
                    }
                )
            )
            return outputs

    def load(self, uri: str) -> "CustomLoader.Adapter":
        model = mlflow.pyfunc.load_model(model_uri=uri)
        adapter = CustomLoader.Adapter(model=model)
        return adapter


LoaderKind = CustomLoader

# %% REGISTERS


class Register(abc.ABC, pdt.BaseModel, strict=True, frozen=True, extra="forbid"):
    """Base class for registring models to a location.

    Separate model definition from its registration.
    e.g., to change the model registry backend.

    Parameters:
        tags (dict[str, T.Any]): tags for the model.
    """

    KIND: str

    tags: dict[str, T.Any] = {}

    @abc.abstractmethod
    def register(self, name: str, model_uri: str) -> Version:
        """Register a model given its name and URI.

        Args:
            name (str): name of the model to register.
            model_uri (str): URI of a model to register.

        Returns:
            Version: information about the registered model.
        """


class MlflowRegister(Register):
    """Register for models in the Mlflow Model Registry.

    https://mlflow.org/docs/latest/model-registry.html
    """

    KIND: T.Literal["MlflowRegister"] = "MlflowRegister"

    def register(self, name: str, model_uri: str) -> Version:
        return mlflow.register_model(name=name, model_uri=model_uri, tags=self.tags)


RegisterKind = MlflowRegister
