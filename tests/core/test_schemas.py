# test_schemas.py

import pandas as pd
import pandera.errors as pa_errors
import pytest
from autogen_team.core.schemas import (
    FeatureImportancesSchema,
    InputsSchema,
    MetadataSchema,
    OutputsSchema,
    TargetsSchema,
)


def test_inputs_schema_valid() -> None:
    """Test valid input data for InputsSchema."""
    valid_data = pd.DataFrame({"input": ["Valid input string"]})
    validated_data = InputsSchema.check(valid_data)
    assert not validated_data.empty


def test_inputs_schema_invalid() -> None:
    """Test invalid input data for InputsSchema."""
    invalid_data = pd.DataFrame({"invalid_input": [12345]})  # Non-string input
    with pytest.raises(pa_errors.SchemaError):
        InputsSchema.check(invalid_data)


def test_outputs_schema_valid() -> None:
    """Test valid output data for OutputsSchema."""
    valid_data = pd.DataFrame(
        {
            "response": ["Generated response"],
            "metadata": [{"timestamp": "2025-01-15T12:00:00Z", "model_version": "v1.0.0"}],
        }
    )
    validated_data = OutputsSchema.check(valid_data)
    assert not validated_data.empty


def test_outputs_schema_invalid_metadata() -> None:
    """Test invalid metadata in OutputsSchema."""
    invalid_data = pd.DataFrame(
        {
            "response": ["Generated response"],
            "metadata_invalid": ["Invalid metadata format"],  # Should be a dict-like structure
        }
    )
    with pytest.raises(pa_errors.SchemaError):
        OutputsSchema.check(invalid_data)


def test_metadata_schema_valid() -> None:
    """Test valid metadata validation using MetadataSchema."""
    valid_metadata = pd.DataFrame(
        {"timestamp": ["2025-01-15T12:00:00Z"], "model_version": ["v1.0.0"]}
    )
    validated_metadata = MetadataSchema.check(valid_metadata)
    assert not validated_metadata.empty


def test_metadata_schema_invalid_timestamp() -> None:
    """Test invalid metadata using MetadataSchema."""
    invalid_metadata = pd.DataFrame(
        {"timestamp_invalid": [12345], "model_version": ["v1.0.0"]}  # Non-string timestamp
    )
    with pytest.raises(pa_errors.SchemaError):
        MetadataSchema.check(invalid_metadata)


def test_metadata_schema_invalid_model_version() -> None:
    """Test invalid metadata using MetadataSchema."""
    invalid_metadata = pd.DataFrame(
        {"timestamp": [12345], "model_version_invalid": ["v1.0.0"]}  # Non-string timestamp
    )
    with pytest.raises(pa_errors.SchemaError):
        MetadataSchema.check(invalid_metadata)


def test_targets_schema_valid() -> None:
    """Test valid target data for TargetsSchema."""
    valid_data = pd.DataFrame({"input_target": ["Sample input"], "response": ["Sample prediction"]})
    validated_data = TargetsSchema.check(valid_data)
    assert not validated_data.empty


def test_targets_schema_invalid() -> None:
    """Test invalid target data for TargetsSchema."""
    invalid_data = pd.DataFrame({"input": ["Valid input"]})  # Missing 'prediction' column
    with pytest.raises(pa_errors.SchemaError):
        TargetsSchema.check(invalid_data)


def test_feature_importances_schema_valid() -> None:
    """Test valid data for FeatureImportancesSchema."""
    valid_data = pd.DataFrame({"feature": ["Feature A"], "importance": [0.5]})
    validated_data = FeatureImportancesSchema.check(valid_data)
    assert not validated_data.empty


def test_feature_importances_schema_invalid() -> None:
    """Test invalid data for FeatureImportancesSchema."""
    invalid_data = pd.DataFrame(
        {
            "feature": [123],
            "importance": ["Invalid importance"],
        }  # Non-string feature, non-float importance
    )
    with pytest.raises(pa_errors.SchemaError):
        FeatureImportancesSchema.check(invalid_data)
