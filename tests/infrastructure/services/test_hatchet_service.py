"""Tests for Hatchet Service."""

import pytest_mock as pm
from autogen_team.infrastructure import services


def test_hatchet_service_initialization(mocker: pm.MockerFixture) -> None:
    # given
    # Mock the Hatchet client to avoid actual connection attempts
    mock_hatchet = mocker.patch("autogen_team.infrastructure.services.hatchet_service.Hatchet")
    service = services.HatchetService(token="test_token", namespace="test_ns")

    # when
    client = service.client

    # then
    assert client is not None
    mock_hatchet.assert_called_once()


def test_hatchet_service_singleton_behavior(mocker: pm.MockerFixture) -> None:
    # given
    mocker.patch("autogen_team.infrastructure.services.hatchet_service.Hatchet")
    service = services.HatchetService()

    # when
    client1 = service.client
    client2 = service.client

    # then
    assert client1 is client2
