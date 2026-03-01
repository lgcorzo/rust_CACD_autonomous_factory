"""Tests for MCP server bootstrap."""

from __future__ import annotations

import json
import os
from contextlib import asynccontextmanager  # Added
from typing import AsyncGenerator, Tuple, Any  # Added
from unittest.mock import AsyncMock, patch, MagicMock

import pytest
from autogen_team.application.mcp.mcp_server import (
    create_mcp_server,
    handle_call_tool,
    handle_list_tools,
)

# ... (rest of your existing tests) ...


@pytest.mark.asyncio
async def test_run_stdio() -> None:
    """Test run_stdio executes stdio_server block."""
    from autogen_team.application.mcp.mcp_server import run_stdio

    with patch("autogen_team.application.mcp.mcp_server._server") as mock_server:
        mock_server.run = AsyncMock()
        mock_server.create_initialization_options.return_value = {}

        # Mypy now recognizes these types from the top-level imports
        @asynccontextmanager
        async def mock_stdio() -> AsyncGenerator[Tuple[str, str], None]:
            yield ("read", "write")

        with patch("autogen_team.application.mcp.mcp_server.stdio_server", side_effect=mock_stdio):
            await run_stdio()

        mock_server.run.assert_called_once_with("read", "write", {})
