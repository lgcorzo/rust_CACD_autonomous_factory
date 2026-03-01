"""Tests for MCPClient."""

import pytest
from typing import Generator, Any, Dict, Union
from unittest.mock import MagicMock, patch, AsyncMock
from autogen_team.infrastructure.client.mcp_client import MCPClient


@pytest.fixture
def mcp_client() -> Generator[MCPClient, None, None]:
    """Fixture to provide an MCPClient instance."""
    yield MCPClient()


@pytest.mark.asyncio
async def test_mcp_client_connect_success(mcp_client: MCPClient) -> None:
    with (
        patch("autogen_team.infrastructure.client.mcp_client.stdio_client") as mock_stdio,
        patch("autogen_team.infrastructure.client.mcp_client.ClientSession") as mock_session_cls,
    ):
        mock_read = MagicMock()
        mock_write = MagicMock()
        mock_client_context = AsyncMock()
        # Mocking the async context manager protocol
        mock_client_context.__aenter__.return_value = (mock_read, mock_write)
        mock_stdio.return_value = mock_client_context

        mock_session = AsyncMock()
        mock_session_cls.return_value = mock_session

        await mcp_client.connect()

        assert mcp_client.session == mock_session
        mock_stdio.assert_called_once()
        mock_session.initialize.assert_called_once()


@pytest.mark.asyncio
async def test_mcp_client_disconnect(mcp_client: MCPClient) -> None:
    mcp_client.session = AsyncMock()
    # Assuming _client_context is the attribute storing the context manager
    mcp_client._client_context = AsyncMock()

    await mcp_client.disconnect()

    mcp_client.session.__aexit__.assert_called_once()
    if mcp_client._client_context:
        mcp_client._client_context.__aexit__.assert_called_once()


@pytest.mark.asyncio
async def test_mcp_client_call_tool_success(mcp_client: MCPClient) -> None:
    mcp_client.session = AsyncMock()
    mock_result = MagicMock()
    mock_result.content = [MagicMock(text='{"status": "ok"}')]
    mcp_client.session.call_tool.return_value = mock_result

    result = await mcp_client.call_tool("test_tool", {"arg": "val"})

    assert result == {"status": "ok"}
    mcp_client.session.call_tool.assert_called_with("test_tool", {"arg": "val"})


@pytest.mark.asyncio
async def test_mcp_client_call_tool_not_json(mcp_client: MCPClient) -> None:
    mcp_client.session = AsyncMock()
    mock_result = MagicMock()
    mock_result.content = [MagicMock(text="plain text")]
    mcp_client.session.call_tool.return_value = mock_result

    result = await mcp_client.call_tool("test_tool", {})

    assert result == "plain text"


@pytest.mark.asyncio
async def test_mcp_client_call_tool_no_session(mcp_client: MCPClient) -> None:
    with patch.object(mcp_client, "connect", new_callable=AsyncMock) as mock_connect:
        mcp_client.session = None

        # Side effect must also be typed for mypy in some configurations
        async def mock_connect_side_effect() -> None:
            mcp_client.session = AsyncMock()
            mock_result = MagicMock()
            mock_result.content = []
            mcp_client.session.call_tool.return_value = mock_result

        mock_connect.side_effect = mock_connect_side_effect

        await mcp_client.call_tool("test_tool", {})

        mock_connect.assert_called_once()


@pytest.mark.asyncio
async def test_mcp_client_call_tool_runtime_error(mcp_client: MCPClient) -> None:
    with patch.object(mcp_client, "connect", new_callable=AsyncMock) as mock_connect:
        mcp_client.session = None
        with pytest.raises(RuntimeError, match="Failed to connect to MCP Server"):
            await mcp_client.call_tool("test_tool", {})
