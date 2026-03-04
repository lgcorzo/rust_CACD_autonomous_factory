"""Tests for MCP server bootstrap."""

from __future__ import annotations

import json
from contextlib import asynccontextmanager  # Added
from typing import AsyncGenerator, Tuple, Any  # Added
from unittest.mock import AsyncMock, patch, MagicMock

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
    """Test that list_tools returns all 7 tools."""
    tools = await handle_list_tools()
    assert len(tools) == 7

    tool_names = {t.name for t in tools}
    expected = {
        "plan_mission",
        "execute_code",
        "run_tests",
        "security_review",
        "retrieve_context",
        "index_code",
        "generate_mission_docs",
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
        result = await handle_call_tool(
            "execute_code", {"task": {"id": 1}, "workspace_path": "/tmp"}
        )

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

    with (
        patch("argparse.ArgumentParser.parse_args") as mock_args,
        patch("autogen_team.application.mcp.mcp_server.create_sse_app") as mock_sse,
        patch("uvicorn.run") as mock_run,
    ):
        mock_args.return_value = MagicMock(transport="sse", host="127.0.0.1", port=8100)
        main()
    mock_sse.assert_called_once()
    mock_run.assert_called_once()


def test_mcp_server_main_stdio() -> None:
    """Test main with stdio transport."""
    from autogen_team.application.mcp.mcp_server import main

    with (
        patch("argparse.ArgumentParser.parse_args") as mock_args,
        patch("asyncio.run") as mock_asyncio_run,
    ):
        mock_args.return_value = MagicMock(transport="stdio")
        main()
    mock_asyncio_run.assert_called_once()


def test_mcp_server_main_default() -> None:
    """Test main with no arguments defaults to sse transport."""
    from autogen_team.application.mcp.mcp_server import main

    with (
        patch("sys.argv", ["mcp_server.py"]),
        patch("autogen_team.application.mcp.mcp_server.create_sse_app") as mock_sse,
        patch("uvicorn.run") as mock_run,
    ):
        main()
    mock_sse.assert_called_once()
    mock_run.assert_called_once()


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


@pytest.mark.asyncio
async def test_call_tool_security_review() -> None:
    """Test call_tool dispatches to security_review correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.security_review", new_callable=AsyncMock
    ) as mock:
        mock.return_value = {"review": "safe"}
        result = await handle_call_tool("security_review", {"diff": "some diff"})
    data = json.loads(result[0].text)
    assert data["review"] == "safe"
    mock.assert_called_once_with(diff="some diff")


@pytest.mark.asyncio
async def test_call_tool_retrieve_context() -> None:
    """Test call_tool dispatches to retrieve_context correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.retrieve_context", new_callable=AsyncMock
    ) as mock:
        mock.return_value = {"context": "found"}
        result = await handle_call_tool("retrieve_context", {"query": "test query"})
    data = json.loads(result[0].text)
    assert data["context"] == "found"
    mock.assert_called_once_with(query="test query")


@pytest.mark.asyncio
async def test_call_tool_index_code() -> None:
    """Test call_tool dispatches to index_code correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.index_code", new_callable=AsyncMock
    ) as mock:
        mock.return_value = {"indexed": True}
        result = await handle_call_tool("index_code", {"file_path": "f.py", "content": "print(1)"})
    data = json.loads(result[0].text)
    assert data["indexed"] is True
    mock.assert_called_once_with(file_path="f.py", content="print(1)")


@pytest.mark.asyncio
async def test_sse_asgi_connect() -> None:
    """Test the SSE ASGI connector block."""
    from autogen_team.application.mcp.mcp_server import create_sse_app

    # We need to mock SseServerTransport so that connect_sse returns a mock stream
    mock_sse_transport = MagicMock()

    @asynccontextmanager
    async def mock_connect(
        scope: Any, receive: Any, send: Any
    ) -> AsyncGenerator[Tuple[str, str], None]:
        yield ("read_stream", "write_stream")

    mock_sse_transport.connect_sse = mock_connect

    with patch(
        "autogen_team.application.mcp.mcp_server.SseServerTransport",
        return_value=mock_sse_transport,
    ):
        app = create_sse_app()
        # Find the route that matches /sse or /sse/
        sse_mount = None
        for r in app.routes:
            path = getattr(r, "path", "")
            if path.rstrip("/") == "/sse":
                sse_mount = r
                break

        assert (
            sse_mount is not None
        ), f"Could not find /sse mount in routes: {[getattr(r, 'path', '') for r in app.routes]}"
        sse_asgi = sse_mount.app

        with patch(
            "autogen_team.application.mcp.mcp_server._server.run", new_callable=AsyncMock
        ) as mock_run:
            await sse_asgi({"type": "http"}, AsyncMock(), AsyncMock())
            mock_run.assert_called_once()


@pytest.mark.asyncio
async def test_call_tool_generate_mission_docs() -> None:
    """Test call_tool dispatches to generate_mission_docs correctly."""
    with patch(
        "autogen_team.application.mcp.mcp_server.generate_mission_docs",
        new_callable=AsyncMock,
    ) as mock:
        mock.return_value = {"summary": "Done"}
        result = await handle_call_tool(
            "generate_mission_docs",
            {"mission_id": "m1", "mission_context": {}},
        )
    data = json.loads(result[0].text)
    assert data["summary"] == "Done"
    mock.assert_called_once_with(mission_id="m1", mission_context={})
