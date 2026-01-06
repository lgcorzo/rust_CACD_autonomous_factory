# %% IMPORTS
from unittest.mock import MagicMock, patch

import pandas as pd
import pytest
from agent_framework.openai import OpenAIChatClient
from autogen_team.core import schemas
from autogen_team.core.models import BaselineAutogenModel


@pytest.fixture
def baseline_model() -> BaselineAutogenModel:
    """Fixture to create an instance of BaselineAutogenModel."""
    return BaselineAutogenModel()


def test_get_params(baseline_model: BaselineAutogenModel) -> None:
    """Test the get_params method."""
    # Setup
    baseline_model.model_config_path = "Newpath"

    # Execute
    params = baseline_model.get_params()

    # Verify
    assert "model_config_path" in params
    assert params["model_config_path"] == "Newpath"


def test_set_params(baseline_model: BaselineAutogenModel) -> None:
    """Test the set_params method."""
    # Setup
    new_params = {"model_config_path": "Newpath"}

    # Execute
    baseline_model.set_params(**new_params)

    # Verify
    assert baseline_model.model_config_path == "Newpath"


def test_predict(baseline_model: BaselineAutogenModel) -> None:
    """Test the predict method of BaselineAutogenModel."""
    # Setup
    input_data = pd.DataFrame({"input": ["Some large input string"]})
    inputs = schemas.Inputs(input_data)

    with patch("autogen_team.core.models.asyncio.run") as MockModelrun:
        # Mock the response
        mock_msg = MagicMock()
        mock_msg.text = "Result 1"
        mock_response = MagicMock()
        mock_response.messages = [mock_msg]
        mock_response.text = "Result 1"
        mock_response.finish_reason = "stop"
        MockModelrun.return_value = mock_response

        # Execute the predict function (await is needed since predict must be async)
        outputs_df: pd.DataFrame = baseline_model.predict(inputs)

        # Check the outputs
        assert outputs_df is not None
        assert "Result 1" in outputs_df["response"][0]
        assert isinstance(outputs_df["metadata"][0], dict), "Metadata should be a dictionary"
        assert "timestamp" in outputs_df["metadata"][0], "Metadata timestamp is missing"
        assert "model_version" in outputs_df["metadata"][0], "Metadata model_version is missing"
        assert "terminated" in outputs_df["metadata"][0], "Metadata terminated is missing"
        assert outputs_df["metadata"][0]["terminated"] is True
        assert "messages" in outputs_df["metadata"][0], "Metadata messages is missing"
        assert outputs_df["metadata"][0]["messages"] == ["Result 1"]


def test_get_internal_model(baseline_model: BaselineAutogenModel) -> None:
    """Test get_internal_model returns the team."""
    # Setup
    mock_team = MagicMock(spec=OpenAIChatClient)
    baseline_model._model_client = mock_team

    # Execute
    internal_model = baseline_model.get_internal_model()

    # Verify
    assert internal_model == mock_team, "get_internal_model should return the team attribute"


def test_load_context(baseline_model: BaselineAutogenModel) -> None:
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
    with patch("autogen_team.core.models.OpenAIChatClient") as MockOpenAIChatClient:
        # Execute
        baseline_model.load_context(model_config)
        # Verify
        MockOpenAIChatClient.assert_called_once()  # Verify AssistantAgent was called
