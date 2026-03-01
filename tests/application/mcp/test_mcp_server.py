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
async def test_call_tool_execute_code() -> None:
    """Test call_tool dispatches to execute_code correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.execute_code",
        new_callable=AsyncMock,
    ) as mock_ec:
        mock_ec.return_value = {"status": "success"}
        result = await handle_call_tool("execute_code", {"task": {"id": 1}, "workspace_path": "/tmp"})

    data = json.loads(result[0].text)
    assert data["status"] == "success"
    mock_ec.assert_called_once_with(task={"id": 1}, workspace_path="/tmp")

@pytest.mark.asyncio
async def test_call_tool_run_tests() -> None:
    """Test call_tool dispatches to run_tests correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.run_tests",
        new_callable=AsyncMock,
    ) as mock_rt:
        mock_rt.return_value = {"passed": True}
        result = await handle_call_tool("run_tests", {"changes": {}, "workspace_path": "/tmp"})

    data = json.loads(result[0].text)
    assert data["passed"] is True
    mock_rt.assert_called_once_with(changes={}, workspace_path="/tmp")

@pytest.mark.asyncio
async def test_call_tool_exception_handling() -> None:
    """Test call_tool handles exceptions gracefully."""
    with patch(
        "autogen_team.application.mcp.mcp_server.plan_mission",
        side_effect=Exception("Tool crash"),
    ):
        result = await handle_call_tool("plan_mission", {"goal": "test"})

    data = json.loads(result[0].text)
    assert "error" in data
    assert "Tool crash" in data["error"]
    assert "traceback" in data

@pytest.mark.asyncio
async def test_create_sse_app() -> None:
    """Test create_sse_app returns a Starlette instance."""
    from autogen_team.application.mcp.mcp_server import create_sse_app
    app = create_sse_app()
    assert app is not None
    assert any(route.path == "/health" for route in app.routes)

def test_mcp_server_main_help() -> None:
    """Test main with --help."""
    from autogen_team.application.mcp.mcp_server import main
    with patch("argparse.ArgumentParser.parse_args") as mock_args, \
         patch("autogen_team.application.mcp.mcp_server.create_sse_app") as mock_sse, \
         patch("uvicorn.run") as mock_run:
        mock_args.return_value = MagicMock(transport="sse", host="127.0.0.1", port=8100)
        main()
    mock_sse.assert_called_once()
    mock_run.assert_called_once()

def test_mcp_server_main_stdio() -> None:
    """Test main with stdio transport."""
    from autogen_team.application.mcp.mcp_server import main
    with patch("argparse.ArgumentParser.parse_args") as mock_args, \
         patch("asyncio.run") as mock_asyncio_run:
        mock_args.return_value = MagicMock(transport="stdio")
        main()
    mock_asyncio_run.assert_called_once()
