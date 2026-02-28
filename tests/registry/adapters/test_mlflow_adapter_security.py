"""Security regression tests for MLflow adapters."""

import os
import pickle
import typing as T
import unittest.mock

from autogen_team.models import entities as models
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver


# Create a dummy model class to avoid pickling issues with MagicMock
class DummyModel(models.Model):
    KIND: T.Literal["DummyModel"] = "DummyModel"

    def load_context(self, model_config: dict[str, T.Any]) -> None:
        pass

    def fit(self, inputs: T.Any, targets: T.Any) -> T.Self:
        return self

    def predict(self, inputs: T.Any) -> T.Any:
        return None


def test_custom_saver_adapter_does_not_capture_env_vars() -> None:
    """Test that CustomSaver.Adapter does not capture LITELLM_API_KEY from env."""
    # given
    secret_key = "secret_api_key_123"
    with unittest.mock.patch.dict(os.environ, {"LITELLM_API_KEY": secret_key}):
        # when
        adapter = CustomSaver.Adapter(model=DummyModel())

        # then
        # Check if model_config exists and contains the key
        if hasattr(adapter, "model_config"):
            config = adapter.model_config
            if "config" in config:
                captured_key = config["config"].get("api_key")
                assert (
                    captured_key != secret_key
                ), "LITELLM_API_KEY leaked into model configuration!"


def test_adapter_does_not_pickle_secrets() -> None:
    """Test that the adapter does not pickle secrets into the model artifact."""
    # Arrange: Set a secret in the environment
    secret_key = "SECRET_API_KEY_123"
    with unittest.mock.patch.dict(os.environ, {"LITELLM_API_KEY": secret_key}):
        # Act: Instantiate the Adapter
        adapter = CustomSaver.Adapter(model=DummyModel())

        # Assert: Verify that the secret is NOT present in the adapter's state
        # We pickle and unpickle to simulate MLflow saving/loading process
        pickled_adapter = pickle.dumps(adapter)
        restored_adapter = pickle.loads(pickled_adapter)

        # Check if model_config exists and contains the secret
        if hasattr(restored_adapter, "model_config"):
            config = restored_adapter.model_config
            # If model_config exists, it MUST NOT contain the secret
            if "config" in config and "api_key" in config["config"]:
                assert (
                    config["config"]["api_key"] != secret_key
                ), "CRITICAL: API Key was pickled with the adapter!"

        # Also check __dict__ just in case it's stored under another name
        for key, value in restored_adapter.__dict__.items():
            if isinstance(value, (str, dict, list)):
                assert secret_key not in str(
                    value
                ), f"CRITICAL: LITELLM_API_KEY captured in adapter attribute '{key}'!"
