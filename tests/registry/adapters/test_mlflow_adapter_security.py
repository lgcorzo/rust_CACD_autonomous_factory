# %% IMPORTS

import os
import pickle
import typing as T
from unittest.mock import MagicMock

import pytest
from autogen_team.core import schemas
from autogen_team.models import entities as models
from autogen_team.registry.adapters import mlflow_adapter as registries

# %% TEST MODEL


class DummyModel(models.Model):
    KIND: str = "DummyModel"

    def predict(self, inputs: schemas.Inputs) -> schemas.Outputs:
        return schemas.Outputs()

    def load_context(self, model_config: T.Dict[str, T.Any]) -> None:
        pass

    def fit(self, inputs: schemas.Inputs, targets: schemas.Targets) -> T.Self:
        return self


# %% TESTS


def test_custom_saver_adapter_does_not_pickle_secrets() -> None:
    """Test that the CustomSaver.Adapter does not capture and pickle environment secrets."""
    # given
    secret_key = "sk-fake-secret-12345"
    os.environ["LITELLM_API_KEY"] = secret_key
    model = DummyModel()

    try:
        # when
        adapter = registries.CustomSaver.Adapter(model=model)

        # Serialize the adapter
        serialized = pickle.dumps(adapter)

        # Unset the secret from environment
        del os.environ["LITELLM_API_KEY"]

        # Deserialize the adapter
        deserialized_adapter = pickle.loads(serialized)

        # then
        # Check if model_config exists and contains the secret
        captured_key = None
        if hasattr(deserialized_adapter, "model_config"):
            config = deserialized_adapter.model_config
            if isinstance(config, dict) and "config" in config:
                captured_key = config["config"].get("api_key")

        assert (
            captured_key != secret_key
        ), "CRITICAL: LITELLM_API_KEY was captured in pickled adapter!"

    finally:
        # Cleanup
        if "LITELLM_API_KEY" in os.environ:
            del os.environ["LITELLM_API_KEY"]
