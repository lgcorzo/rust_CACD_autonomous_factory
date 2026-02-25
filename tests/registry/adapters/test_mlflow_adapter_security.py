"""Security regression tests for MLflow adapters."""

import os
import unittest.mock


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
                assert (
                    captured_key != secret_key
                ), "LITELLM_API_KEY leaked into model configuration!"


class TestMlflowAdapterSecurity(unittest.TestCase):
    """Test suite for security aspects of MLflow adapters."""

    def test_custom_saver_adapter_does_not_capture_secrets(self) -> None:
        """
        Vulnerability Check: Ensure CustomSaver.Adapter does NOT capture
        environment variables (like LITELLM_API_KEY) in its state during initialization.
        """
        # Given: A secret is present in the environment
        secret_value = "SUPER_SECRET_VALUE"
        os.environ["LITELLM_API_KEY"] = secret_value

        # And: A mock model
        mock_model = unittest.mock.MagicMock()

        # When: The adapter is initialized
        adapter = CustomSaver.Adapter(model=mock_model)

        # Then: The adapter should NOT have 'model_config' attribute containing the secret
        # Note: If model_config exists, it must NOT contain the secret.
        # Ideally, it shouldn't exist if it's dead code.

        if hasattr(adapter, "model_config"):
            self.assertNotIn(
                secret_value,
                str(adapter.model_config),
                "CRITICAL: LITELLM_API_KEY captured in adapter.model_config! This will be pickled with the model.",
            )

        # Also check __dict__ just in case it's stored under another name
        for key, value in adapter.__dict__.items():
            if isinstance(value, (str, dict, list)):
                self.assertNotIn(
                    secret_value,
                    str(value),
                    f"CRITICAL: LITELLM_API_KEY captured in adapter attribute '{key}'!",
                )

        # Cleanup
        del os.environ["LITELLM_API_KEY"]


if __name__ == "__main__":
    unittest.main()
