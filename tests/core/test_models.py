# %% IMPORTS
import pytest
from unittest.mock import MagicMock, patch, AsyncMock
import pandas as pd
import typing as T

from autogen_agentchat.base import TaskResult
from autogen_team.core.models import BaselineAutogenModel, Model
from autogen_team.core import schemas


@pytest.fixture
def baseline_model():
    """Fixture to create an instance of BaselineAutogenModel."""
    return BaselineAutogenModel()


def test_get_params(baseline_model):
    """Test the get_params method."""
    # Setup
    baseline_model.model_config_path = "Newpath"

    # Execute
    params = baseline_model.get_params()

    # Verify
    assert "model_config_path" in params
    assert params["model_config_path"] == "Newpath"


def test_set_params(baseline_model):
    """Test the set_params method."""
    # Setup
    new_params = {"model_config_path": "Newpath"}

    # Execute
    baseline_model.set_params(**new_params)

    # Verify
    assert baseline_model.model_config_path == "Newpath"


@pytest.fixture
def async_response_stream():
    return MagicMock(content="Result 1")


def test_predict(baseline_model, async_response_stream):
    """Test the predict method of BaselineAutogenModel."""
    # Setup
    input_data = pd.DataFrame({"input": ["Some large input string"]})
    inputs = schemas.Inputs(input_data)

    with patch("autogen_team.core.models.asyncio.run") as MockModelrun:
        # Mock the create method to return our async generator
        MockModelrun.return_value = async_response_stream

        # Execute the predict function (await is needed since predict must be async)
        outputs = baseline_model.predict(inputs)

        # Check the outputs
        assert outputs is not None
        assert "Result 1" in outputs["response"][0]

        # Additional metadata validation (optional)
        assert isinstance(outputs["metadata"][0], dict), "Metadata should be a dictionary"
        assert "timestamp" in outputs["metadata"][0], "Metadata timestamp is missing"
        assert "model_version" in outputs["metadata"][0], "Metadata model_version is missing"


def test_get_internal_model(baseline_model):
    """Test get_internal_model returns the team."""
    # Setup
    mock_team = MagicMock()
    baseline_model.model_client = mock_team

    # Execute
    internal_model = baseline_model.get_internal_model()

    # Verify
    assert internal_model == mock_team, "get_internal_model should return the team attribute"


def test_model_class_config():
    """Test the Config settings for the Model class."""

    class CustomModel(Model):
        KIND: T.Literal["CustomModel"] = "CustomModel"

        @T.override
        def load_context(self, model_config):
            pass

    # Verify Config
    assert CustomModel.Config.arbitrary_types_allowed is True


def test_load_context(baseline_model):
    # Setup
    model_config = {
        "provider": "openai_chat_completion_client",  # Use LiteLLM-compatible client
        "config": {
            "model": "azure-gpt",  # LiteLLM model
            "api_base": "https://localhost:4000",  # LiteLLM Gateway URL
            "api_key": "sk-12345",
            "temperature": 0.7,  # Optional
            "max_tokens": 512,  # Optional
        },
    }  # Provide your model config as necessary
    with patch(
        "autogen_team.core.models.OpenAIChatCompletionClient"
    ) as MockOpenAIChatCompletionClient:
        # Execute
        baseline_model.load_context(model_config)
        # Verify
        MockOpenAIChatCompletionClient.assert_called_once()  # Verify AssistantAgent was called
