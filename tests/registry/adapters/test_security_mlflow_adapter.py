import os
import pytest
from unittest.mock import MagicMock
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver

class TestSecurityMlflowAdapter:
    def test_adapter_does_not_capture_secrets_in_init(self) -> None:
        """
        Verify that CustomSaver.Adapter does NOT capture environment variables (secrets)
        during initialization, preventing accidental pickling of secrets.
        """
        # Set a dummy secret in the environment
        secret_value = "super_secret_value_12345"
        os.environ["LITELLM_API_KEY"] = secret_value

        try:
            # Mock the internal model as it's required by __init__
            mock_model = MagicMock()

            # Instantiate the adapter
            adapter = CustomSaver.Adapter(model=mock_model)

            # Check for the vulnerability:
            # The vulnerable code stores 'model_config' in self with the secret
            if hasattr(adapter, "model_config"):
                config = getattr(adapter, "model_config", {})
                if isinstance(config, dict):
                    api_key = config.get("config", {}).get("api_key")
                    # Assert that the API key is NOT captured
                    # This assertion will FAIL if the vulnerability exists
                    assert api_key != secret_value, "VULNERABILITY DETECTED: API Key captured in adapter.model_config!"

            # If model_config doesn't exist (after fix), the test passes for this check.

        finally:
            # Clean up environment
            if "LITELLM_API_KEY" in os.environ:
                del os.environ["LITELLM_API_KEY"]
