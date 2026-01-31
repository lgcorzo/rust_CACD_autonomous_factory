"""Tests for Hatchet Inference Job."""

import pytest_mock as pm
from autogen_team.application.jobs.hatchet_inference import HatchetInferenceJob
from autogen_team.data_access.adapters import datasets
from autogen_team.infrastructure import services
from autogen_team.registry.adapters import mlflow_adapter as registries


def test_hatchet_inference_job_trigger(
    mocker: pm.MockerFixture,
    mlflow_service: services.MlflowService,
    alerts_service: services.AlertsService,
    logger_service: services.LoggerService,
    inputs_reader: datasets.ParquetReader,
    tmp_outputs_writer: datasets.ParquetWriter,
    loader: registries.CustomLoader,
) -> None:
    # given
    # Mock HatchetService and its client
    mock_hatchet_service = mocker.Mock(spec=services.HatchetService)
    mock_client = mocker.Mock()
    mock_hatchet_service.client = mock_client

    # when
    job = HatchetInferenceJob(
        logger_service=logger_service,
        alerts_service=alerts_service,
        mlflow_service=mlflow_service,
        inputs=inputs_reader,
        outputs=tmp_outputs_writer,
        alias_or_version="Champion",
        loader=loader,
        hatchet_service=mock_hatchet_service,
    )

    with job as runner:
        out = runner.run()

    # then
    # Verify that admin.run_workflow was called with correct parameters
    mock_client.admin.run_workflow.assert_called_once()
    args, _ = mock_client.admin.run_workflow.call_args
    assert args[0] == "InferenceWorkflow"

    workflow_input = args[1]
    assert workflow_input["alias_or_version"] == "Champion"
    assert "inputs" in workflow_input
    assert "outputs" in workflow_input
    assert "loader" in workflow_input

    # Verify return variables
    assert "workflow_input" in out
    assert out["self"] == job
