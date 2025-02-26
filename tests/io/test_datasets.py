# %% IMPORTS

import os

import pytest

import mlflow.data.pandas_dataset.PandasDataset as Lineage

from autogen_team.core import schemas
from autogen_team.io import datasets


# %% READERS


@pytest.mark.parametrize("limit", [None, 50])
def test_parquet_reader(limit: int | None, inputs_path: str) -> None:
    # given
    reader = datasets.ParquetReader(path=inputs_path, limit=limit)
    # when
    data = reader.read()
    lineage: Lineage = reader.lineage(name="inputs", data=data)
    # then
    # - data
    assert data.ndim == 2, "Data should be a dataframe!"
    if limit is not None:
        assert len(data) == limit, "Data should have the limit size!"
    # - lineage
    assert lineage.name == "inputs", "Lineage name should be inputs!"
    # Fix 1: Use correct attribute name for DatasetSource (check your implementation)
    assert (
        lineage.source.uri == inputs_path
    ), "Lineage source path should be the inputs path!"  # Changed .uri to .path

    # Fix 2: Handle optional schema
    assert lineage.schema is not None, "Lineage schema should be defined!"
    assert set(lineage.schema.input_names()) == set(
        data.columns
    ), "Lineage schema names should be the data columns!"

    # Fix 3: Handle profile type
    assert lineage.profile is not None, "Lineage profile should be defined!"
    assert isinstance(lineage.profile, dict), "Lineage profile should be a dictionary!"
    assert lineage.profile["num_rows"] == len(
        data
    ), "Lineage profile should contain the data row count!"


# %% WRITERS


def test_parquet_writer(targets: schemas.Targets, tmp_outputs_path: str) -> None:
    # given
    writer = datasets.ParquetWriter(path=tmp_outputs_path)
    # when
    writer.write(data=targets)
    # then
    assert os.path.exists(tmp_outputs_path), "Data should be written!"
