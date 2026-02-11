"""MCP Service - Model Context Protocol Server Lifecycle."""

from __future__ import annotations

import typing as T
from typing import ClassVar

import httpx
import litellm
from pydantic import Field

from autogen_team.infrastructure.io.osvariables import Env

from .logger_service import Service


class MCPService(Service):
    """Service for MCP server lifecycle and backend clients.

    Manages LiteLLM and R2R HTTP client initialization, providing
    a single point of access for all MCP tool backends.

    Parameters:
        litellm_api_base (str): LiteLLM API base URL.
        litellm_api_key (str): LiteLLM API key.
        litellm_model (str): Default LiteLLM model identifier.
        r2r_base_url (str): R2R RAG API base URL.
    """

    env: ClassVar[Env] = Env()
    litellm_api_base: str = Field(default_factory=lambda: MCPService.env.litellm_api_base)
    litellm_api_key: str = Field(default_factory=lambda: MCPService.env.litellm_api_key)
    litellm_model: str = Field(default_factory=lambda: MCPService.env.litellm_model)
    r2r_base_url: str = Field(default_factory=lambda: MCPService.env.r2r_base_url)
    prompts_path: str = Field(default_factory=lambda: MCPService.env.mcp_prompts_path)

    _r2r_client: httpx.AsyncClient | None = None
    _prompts: dict[str, T.Any] | None = None

    def start(self) -> None:
        """Initialize LiteLLM configuration and R2R HTTP client."""
        litellm.api_base = self.litellm_api_base
        litellm.api_key = self.litellm_api_key
        self._r2r_client = httpx.AsyncClient(
            base_url=self.r2r_base_url,
            timeout=httpx.Timeout(30.0),
        )
        self._load_prompts()

    def _load_prompts(self) -> None:
        """Load prompts from YAML file."""
        import os

        from autogen_team.infrastructure.io.configs import parse_file, to_object

        if os.path.exists(self.prompts_path):
            config = parse_file(self.prompts_path)
            self._prompts = T.cast(dict[str, T.Any], to_object(config))
        else:
            self._prompts = {}

    def get_prompt(self, tool_name: str, key: str = "system") -> str:
        """Get a specific prompt for a tool and key."""
        if self._prompts is None:
            self._load_prompts()

        tool_prompts = self._prompts.get(tool_name, {}) if self._prompts else {}
        return str(tool_prompts.get(key, ""))

    def stop(self) -> None:
        """Stop the MCP service and close HTTP clients."""
        if self._r2r_client is not None:
            # Schedule close; in sync context just discard reference
            self._r2r_client = None

    @property
    def r2r_client(self) -> httpx.AsyncClient:
        """Return the R2R async HTTP client."""
        if self._r2r_client is None:
            self.start()
        if self._r2r_client is None:
            raise RuntimeError("R2R client failed to initialize.")
        return self._r2r_client
