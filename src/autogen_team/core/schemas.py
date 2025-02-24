# %% IMPORTS

import typing as T

import pandas as pd
import pandera as pa
import pandera.typing as papd
import pandera.typing.common as padt

# %% TYPES

# Generic type for a dataframe container
TSchema = T.TypeVar("TSchema", bound="pa.DataFrameModel")

# %% SCHEMAS


class Schema(pa.DataFrameModel):
    """Base class for a dataframe schema.

    Use a schema to type your dataframe object.
    e.g., to communicate and validate its fields.
    """

    class Config:
        """Default configurations for all schemas.

        Parameters:
            coerce (bool): convert data type if possible.
            strict (bool): ensure the data type is correct.
        """

        coerce: bool = True
        strict: bool = True

    @classmethod
    def check(cls: T.Type[TSchema], data: pd.DataFrame) -> papd.DataFrame[TSchema]:
        """Check the dataframe with this schema.

        Args:
            data (pd.DataFrame): dataframe to check.

        Returns:
            papd.DataFrame[TSchema]: validated dataframe.
        """
        return T.cast(papd.DataFrame[TSchema], cls.validate(data))


class MetadataSchema(Schema):
    """Schema for metadata in outputs."""

    timestamp: papd.Series[padt.String] = pa.Field()
    model_version: papd.Series[padt.String] = pa.Field()


class InputsSchema(Schema):
    """Schema for validating large string inputs."""

    input: papd.Series[padt.String] = pa.Field()


class OutputsSchema(Schema):
    """Schema for structured JSON outputs."""

    response: papd.Series[padt.String] = pa.Field()
    metadata: papd.Series[padt.Object] = pa.Field()


class TargetsSchema(Schema):
    """Schema for the project target."""

    input_target: papd.Series[padt.String] = pa.Field()
    response: papd.Series[padt.String] = pa.Field()


class SHAPValuesSchema(Schema):
    """Schema for SHAP values."""

    sample: papd.Series[padt.String] = pa.Field()
    explanation: papd.Series[padt.String] = pa.Field()
    shap_value: papd.Series[padt.Float32] = pa.Field()

    class Config:
        strict: bool = False


class FeatureImportancesSchema(Schema):
    """Schema for feature importances."""

    feature: papd.Series[padt.String] = pa.Field()
    importance: papd.Series[padt.Float32] = pa.Field()


Inputs = papd.DataFrame[InputsSchema]
Targets = papd.DataFrame[TargetsSchema]
Outputs = papd.DataFrame[OutputsSchema]
SHAPValues = papd.DataFrame[SHAPValuesSchema]
FeatureImportances = papd.DataFrame[FeatureImportancesSchema]

# Test examples to illustrate usage
if __name__ == "__main__":
    # Example for InputSchema validation
    input_data = pd.DataFrame({"input": ["Some large input string"]})
    print("Input validation result:", InputsSchema.check(input_data))

    # Example for OutputSchema validation
    output_data = pd.DataFrame(
        {
            "response": ["Generated output string"],
            "metadata": [{"timestamp": "2025-01-15T12:00:00Z", "model_version": "v1.0.0"}],
        }
    )
    print("Output validation result:", OutputsSchema.check(output_data))
