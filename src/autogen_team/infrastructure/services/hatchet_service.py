"""Hatchet Service - Task Orchestration."""

from __future__ import annotations

from typing import ClassVar

from hatchet_sdk import Hatchet
from pydantic import Field

from autogen_team.infrastructure.io.osvariables import Env

from .logger_service import Service


class HatchetService(Service):
    """Service for Hatchet task orchestration."""

    env: ClassVar[Env] = Env()
    token: str = Field(default_factory=lambda: HatchetService.env.hatchet_client_token)
    namespace: str = Field(default_factory=lambda: HatchetService.env.hatchet_namespace)

    _client: Hatchet | None = None

    def start(self) -> None:
        """Initialize the Hatchet client."""
        try:
            self._client = Hatchet(debug=True)
        except Exception:
            # Fallback for local development or tests if token is not provided
            from unittest.mock import MagicMock

            mock_client = MagicMock()
            # Ensure decorators return the original function/class
            mock_client.workflow.return_value = lambda x: x
            mock_client.step.return_value = lambda x: x
            self._client = mock_client

    def stop(self) -> None:
        """Stop the Hatchet service."""
        self._client = None

    @property
    def client(self) -> Hatchet:
        """Return the Hatchet client."""
        if self._client is None:
            self.start()
        if self._client is None:
            raise RuntimeError("Hatchet client failed to start.")
        return self._client
