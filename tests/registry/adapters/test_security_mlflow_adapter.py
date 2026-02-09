"""Test security of MLflow adapters."""

import os
import pickle
import typing as T
from typing import Any, Dict

import pandas as pd
import pytest

from autogen_team.core import schemas
from autogen_team.models import entities as models
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver


class DummyModel(models.Model):
    """A dummy model for testing."""

    KIND: str = "DummyModel"

    def load_context(self, model_config: Dict[str, Any]) -> None:
        """Load the model context."""
        pass

    def fit(self, inputs: schemas.Inputs, targets: schemas.Targets) -> T.Self:
        """Fit the model."""
        return self

    def predict(self, inputs: schemas.Inputs) -> schemas.Outputs:
        """Predict using the model."""
        return schemas.Outputs(pd.DataFrame())

    def explain_model(self) -> schemas.FeatureImportances:
        """Explain the model."""
        return schemas.FeatureImportances(pd.DataFrame())

    def explain_samples(self, inputs: schemas.Inputs) -> schemas.SHAPValues:
        """Explain samples."""
        return schemas.SHAPValues(pd.DataFrame())

    def get_internal_model(self) -> Any:
        """Get internal model."""
        return None


def test_mlflow_adapter_no_secret_leak() -> None:
    """Test that MLflow adapter does not leak secrets."""
    secret_key = "super-secret-key-123"
    os.environ["LITELLM_API_KEY"] = secret_key

    model = DummyModel()
    adapter = CustomSaver.Adapter(model=model)

    # Check if API key is present in attributes
    if hasattr(adapter, "model_config"):
        config = getattr(adapter, "model_config")
        if isinstance(config, dict) and "config" in config:
            api_key = config["config"].get("api_key")
            assert api_key != secret_key, "API Key leaked in adapter.model_config"

    # Check pickling serialization
    pickled_adapter = pickle.dumps(adapter)
    assert secret_key.encode() not in pickled_adapter, "API Key leaked in pickled adapter"
