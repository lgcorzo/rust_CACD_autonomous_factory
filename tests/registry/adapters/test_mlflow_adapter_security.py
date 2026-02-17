"""Security regression tests for MLflow adapters."""
import os
import unittest.mock
import pytest
from autogen_team.registry.adapters import mlflow_adapter
from autogen_team.models import entities as models

def test_custom_saver_adapter_does_not_capture_env_vars() -> None:
    """Test that CustomSaver.Adapter does not capture LITELLM_API_KEY from env."""
    # given
    secret_key = "secret_api_key_123"
    with unittest.mock.patch.dict(os.environ, {"LITELLM_API_KEY": secret_key}):
        mock_model = unittest.mock.MagicMock(spec=models.Model)

        # when
        adapter = mlflow_adapter.CustomSaver.Adapter(model=mock_model)

        # then
        # Check if model_config exists and contains the key
        if hasattr(adapter, "model_config"):
            config = adapter.model_config
            if "config" in config:
                captured_key = config["config"].get("api_key")
                assert captured_key != secret_key, "LITELLM_API_KEY leaked into model configuration!"
