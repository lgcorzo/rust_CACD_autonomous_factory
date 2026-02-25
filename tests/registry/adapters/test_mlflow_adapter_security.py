"""Security regression tests for MLflow adapters."""

import os
import typing as T
import unittest.mock
from unittest import mock

import pytest

from autogen_team.models import entities as models
from autogen_team.registry.adapters import mlflow_adapter
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver


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


class TestMlflowAdapterSecurity:
    def test_custom_saver_adapter_does_not_capture_secrets(self) -> None:
        """
        Verify that CustomSaver.Adapter does not capture environment variables
        (like LITELLM_API_KEY) in its __init__ method.

        Vulnerability: If the adapter captures env vars in __init__ and stores them
        in self, they will be pickled with the model, leaking secrets into the artifact.
        """
        secret_value = "secret_key_value_12345"

        with mock.patch.dict(os.environ, {"LITELLM_API_KEY": secret_value}):
            # Mock the model argument as it's required
            mock_model = mock.Mock()

            # Instantiate the adapter
            adapter = CustomSaver.Adapter(model=mock_model)

            # Check for the vulnerability
            # If the code is VULNERABLE, adapter.model_config exists and has the secret.

            # We assert that the secret is NOT present in the object state
            # This handles both cases: model_config removed (safe) or present but sanitized (safe).

            if hasattr(adapter, "model_config"):
                config = adapter.model_config
                # Navigate to the key
                api_key = config.get("config", {}).get("api_key")

                # Fail explicitly if the secret is found
                assert api_key != secret_value, (
                    "CRITICAL VULNERABILITY: Adapter captured LITELLM_API_KEY from environment! "
                    "This secret will be pickled with the model artifact."
                )
