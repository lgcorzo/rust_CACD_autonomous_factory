# %% IMPORTS

import os
from unittest.mock import MagicMock
import pytest
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver
from autogen_team.models import entities as models

# %% TESTS


def test_custom_saver_adapter_does_not_capture_secrets() -> None:
    """Test that CustomSaver.Adapter does not capture environment variables in __init__."""
    # given
    secret_key = "fake-secret-key-123"
    os.environ["LITELLM_API_KEY"] = secret_key
    mock_model = MagicMock(spec=models.Model)

    # when
    adapter = CustomSaver.Adapter(model=mock_model)

    # then
    # Ensure model_config is NOT present or does NOT contain the secret
    # If model_config exists, it should not have api_key from env
    if hasattr(adapter, "model_config"):
        config = getattr(adapter, "model_config", {})
        if "config" in config and "api_key" in config["config"]:
            assert config["config"]["api_key"] != secret_key, "API key captured in __init__!"

    # Double check that we didn't just rename it
    for key, value in adapter.__dict__.items():
        if isinstance(value, dict):
            # Recursively check for the secret string
            # We convert value to string to catch it even if deeply nested
            if str(secret_key) in str(value):
                pytest.fail(f"Secret found in {key}!")
