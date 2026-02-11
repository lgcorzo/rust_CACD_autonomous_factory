"""Tests for MCP server bootstrap."""

from __future__ import annotations

import json
from unittest.mock import AsyncMock, patch

import pytest
from autogen_team.application.mcp.mcp_server import (
    create_mcp_server,
    handle_call_tool,
    handle_list_tools,
)


def test_create_mcp_server() -> None:
    """Test that create_mcp_server returns a Server instance."""
    server = create_mcp_server()
    assert server is not None
    assert server.name == "autogen-mcp-server"


@pytest.mark.asyncio
async def test_list_tools_returns_all_tools() -> None:
    """Test that list_tools returns all 6 tools."""
    tools = await handle_list_tools()
    assert len(tools) == 6

    tool_names = {t.name for t in tools}
    expected = {
        "plan_mission",
        "execute_code",
        "run_tests",
        "security_review",
        "retrieve_context",
        "index_code",
    }
    assert tool_names == expected


@pytest.mark.asyncio
async def test_list_tools_have_schemas() -> None:
    """Test that all tools have valid input schemas."""
    tools = await handle_list_tools()
    for tool in tools:
        assert tool.inputSchema is not None
        assert "properties" in tool.inputSchema
        assert "required" in tool.inputSchema


@pytest.mark.asyncio
async def test_call_tool_unknown() -> None:
    """Test call_tool with unknown tool name."""
    result = await handle_call_tool("nonexistent_tool", {})
    assert len(result) == 1
    data = json.loads(result[0].text)
    assert "error" in data
    assert "Unknown tool" in data["error"]


@pytest.mark.asyncio
async def test_call_tool_plan_mission() -> None:
    """Test call_tool dispatches to plan_mission correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.plan_mission",
        new_callable=AsyncMock,
    ) as mock_pm:
        mock_pm.return_value = {"goal": "test", "parallel_tasks": []}
        result = await handle_call_tool("plan_mission", {"goal": "test goal"})

    assert len(result) == 1
    data = json.loads(result[0].text)
    assert data["goal"] == "test"
    mock_pm.assert_called_once_with(goal="test goal")


@pytest.mark.asyncio
async def test_call_tool_security_review() -> None:
    """Test call_tool dispatches to security_review correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.security_review",
        new_callable=AsyncMock,
    ) as mock_sr:
        mock_sr.return_value = {"status": "approved", "findings": []}
        result = await handle_call_tool("security_review", {"diff": "+x = 1"})

    data = json.loads(result[0].text)
    assert data["status"] == "approved"
    mock_sr.assert_called_once_with(diff="+x = 1")
