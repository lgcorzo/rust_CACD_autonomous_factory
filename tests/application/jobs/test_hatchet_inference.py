"""Tests for Hatchet Inference Job."""

import pytest
import pytest_mock as pm
from unittest.mock import patch
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
        runner.run()

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


def test_hatchet_inference_job_failure(
    mocker: pm.MockerFixture,
    mlflow_service: services.MlflowService,
    alerts_service: services.AlertsService,
    logger_service: services.LoggerService,
    inputs_reader: datasets.ParquetReader,
    tmp_outputs_writer: datasets.ParquetWriter,
    loader: registries.CustomLoader,
) -> None:
    # given
    mock_hatchet_service = mocker.Mock(spec=services.HatchetService)
    mock_client = mocker.Mock()
    mock_hatchet_service.client = mock_client
    # Simulate failure
    mock_client.admin.run_workflow.side_effect = Exception("Hatchet error")

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

    # then
    with patch(
        "autogen_team.infrastructure.services.alert_service.AlertsService.notify"
    ) as mock_notify:
        with pytest.raises(Exception, match="Hatchet error"):
            with job as runner:
                runner.run()

        # Verify alerts was notified
        mock_notify.assert_called_with(
            title="Hatchet Inference Failed",
            message="Error triggering workflow: Hatchet error",
        )
