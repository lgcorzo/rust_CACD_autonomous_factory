import os
import pickle
import pytest
from unittest.mock import MagicMock
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver
from autogen_team.models import entities as models
import typing as T


# Create a dummy model class to avoid pickling issues with MagicMock
class DummyModel(models.Model):
    KIND: T.Literal["DummyModel"] = "DummyModel"

    def load_context(self, model_config: dict[str, T.Any]) -> None:
        pass

    def fit(self, inputs: T.Any, targets: T.Any) -> T.Self:
        return self

    def predict(self, inputs: T.Any) -> T.Any:
        return None


def test_adapter_does_not_pickle_secrets() -> None:
    # Arrange: Set a secret in the environment
    secret_key = "SECRET_API_KEY_123"
    os.environ["LITELLM_API_KEY"] = secret_key

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

    # Ideally, model_config should not even exist or be empty of secrets
    # But for this test, we primarily want to ensure the specific secret is not there.
