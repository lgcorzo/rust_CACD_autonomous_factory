"""Tests for Hatchet Orchestration."""

import pytest_mock as pm
from autogen_team.infrastructure.orchestration.hatchet_workflows import InferenceWorkflow


def test_inference_workflow_step(mocker: pm.MockerFixture) -> None:
    # given
    mock_context = mocker.Mock()
    mock_context.workflow_input.return_value = {
        "alias_or_version": "Champion",
        "inputs": {"KIND": "ParquetReader", "path": "test.parquet"},
        "outputs": {"KIND": "ParquetWriter", "path": "results.parquet"},
    }

    # Mock the InferenceJob to avoid actual model loading and inference
    mock_job = mocker.patch("autogen_team.application.jobs.inference.InferenceJob")
    mock_job_instance = mock_job.return_value.__enter__.return_value
    mock_job_instance.run.return_value = {"outputs": mocker.Mock(shape=(10, 2))}

    workflow = InferenceWorkflow()

    # when
    result = workflow.run_inference(mock_context)

    # then
    assert result["status"] == "completed"
    assert "outputs_shape" in result
    mock_job.assert_called_once()
    mock_job_instance.run.assert_called_once()
