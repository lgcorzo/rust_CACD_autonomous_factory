"""Tests for Hatchet Service."""

import pytest_mock as pm
from typing import Any
from unittest.mock import patch
from autogen_team.infrastructure import services


def test_hatchet_service_fallback(mocker: pm.MockerFixture) -> None:
    """Test fallback mock creation when real Hatchet is not used."""
    # Ensure we are in test mode so fallback is triggered
    service = services.HatchetService(token="")

    client = service.client
    assert client is not None
    # Check fallback mock structure
    assert client.admin.run_workflow() == "mock-run-id"

    # Test workflow decorator fallback
    workflow = client.workflow(name="test-workflow")

    @workflow.task()
    def my_task(task_input: Any, context: Any) -> None:
        pass

    assert hasattr(my_task, "fn")


def test_hatchet_service_stop(mocker: pm.MockerFixture) -> None:
    """Test HatchetService.stop."""
    service = services.HatchetService()
    _ = service.client
    assert service._client is not None
    service.stop()
    assert service._client is None


def test_hatchet_service_failure(mocker: pm.MockerFixture) -> None:
    """Test HatchetService property failure when start fails."""
    import pytest

    # Mock start on the class because instances are frozen
    with patch(
        "autogen_team.infrastructure.services.hatchet_service.HatchetService.start",
        return_value=None,
    ):
        service = services.HatchetService()
        with pytest.raises(RuntimeError, match="Hatchet client failed to start"):
            _ = service.client
