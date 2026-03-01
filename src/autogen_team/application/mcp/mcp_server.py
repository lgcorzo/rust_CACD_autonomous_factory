"""MCP Server — Model Context Protocol server bootstrap.

Registers all 6 tools and exposes them via the MCP protocol.
Supports both stdio and SSE transports.
"""

from __future__ import annotations

import argparse
import asyncio
import json
import os
import typing as T

import uvicorn
from mcp.server import Server
from mcp.server.sse import SseServerTransport
from mcp.server.stdio import stdio_server
from mcp.types import TextContent, Tool
from starlette.applications import Starlette
from starlette.requests import Request
from starlette.responses import Response
from starlette.routing import Mount, Route
from starlette.types import Receive, Scope, Send

from autogen_team.application.mcp.tools import (
    execute_code,
    index_code,
    plan_mission,
    retrieve_context,
    run_tests,
    security_review,
)

# Create the MCP server instance
_server = Server("autogen-mcp-server")


def create_mcp_server() -> Server:
    return _server


@_server.list_tools()  # type: ignore[untyped-decorator]
async def handle_list_tools() -> T.List[Tool]:
    """List all available MCP tools."""
    return [
        Tool(
            name="plan_mission",
            description="Decomposes a goal into a task DAG.",
            inputSchema={
                "type": "object",
                "properties": {"goal": {"type": "string"}},
                "required": ["goal"],
            },
        ),
        Tool(
            name="execute_code",
            description="Generates and validates code changes.",
            inputSchema={
                "type": "object",
                "properties": {
                    "task": {"type": "object"},
                    "workspace_path": {"type": "string"},
                },
                "required": ["task", "workspace_path"],
            },
        ),
        Tool(
            name="run_tests",
            description="Runs pytest in isolated sandbox.",
            inputSchema={
                "type": "object",
                "properties": {
                    "changes": {"type": "object"},
                    "workspace_path": {"type": "string"},
                },
                "required": ["changes"],
            },
        ),
        Tool(
            name="security_review",
            description="Analyzes code diffs for security issues.",
            inputSchema={
                "type": "object",
                "properties": {"diff": {"type": "string"}},
                "required": ["diff"],
            },
        ),
        Tool(
            name="retrieve_context",
            description="Queries R2R RAG for patterns.",
            inputSchema={
                "type": "object",
                "properties": {"query": {"type": "string"}},
                "required": ["query"],
            },
        ),
        Tool(
            name="index_code",
            description="Indexes code files into R2R.",
            inputSchema={
                "type": "object",
                "properties": {
                    "file_path": {"type": "string"},
                    "content": {"type": "string"},
                },
                "required": ["file_path", "content"],
            },
        ),
    ]


@_server.call_tool()  # type: ignore[untyped-decorator]
async def handle_call_tool(name: str, arguments: T.Dict[str, T.Any] | None) -> T.List[TextContent]:
    args = arguments or {}
    try:
        if name == "plan_mission":
            result = await plan_mission(goal=args.get("goal", ""))
        elif name == "execute_code":
            result = await execute_code(task=args.get("task", {}), workspace_path=args.get("workspace_path", ""))
        elif name == "run_tests":
            result = await run_tests(changes=args.get("changes", {}), workspace_path=args.get("workspace_path", ""))
        elif name == "security_review":
            result = await security_review(diff=args.get("diff", ""))
        elif name == "retrieve_context":
            result = await retrieve_context(query=args.get("query", ""))
        elif name == "index_code":
            result = await index_code(file_path=args.get("file_path", ""), content=args.get("content", ""))
        else:
            result = {"error": f"Unknown tool: {name}"}
    except Exception as e:
        import traceback

        result = {"error": str(e), "traceback": traceback.format_exc()}

    return [TextContent(type="text", text=json.dumps(result, indent=2))]


async def run_stdio() -> None:
    async with stdio_server() as (read_stream, write_stream):
        await _server.run(read_stream, write_stream, _server.create_initialization_options())


def create_sse_app() -> Starlette:
    sse = SseServerTransport("/sse/messages/")
    from mcp.server.models import InitializationOptions

    async def _sse_asgi(scope: Scope, receive: Receive, send: Send) -> None:
        async with sse.connect_sse(scope, receive, send) as (read_stream, write_stream):
            # Create initialization options with explicit endpoint to avoid path duplication
            init_options = InitializationOptions(
                server_name="autogen-mcp-server",
                server_version="0.1.0",
                capabilities=_server.get_capabilities(),
            )
            await _server.run(read_stream, write_stream, init_options)

    async def health_check(request: Request) -> Response:
        return Response(json.dumps({"status": "healthy"}), media_type="application/json")

    # The updated routing logic:
    routes = [
        # Use Route for standard HTTP GETs
        Route("/", endpoint=health_check, methods=["GET"]),
        Route("/health", endpoint=health_check, methods=["GET"]),
        # Use Mount for the ASGI-based MCP transport
        Mount("/sse/", app=_sse_asgi),
        Mount("/sse/messages/", app=sse.handle_post_message),
    ]
    return Starlette(routes=routes)


def main() -> None:
    parser = argparse.ArgumentParser(description="MCP Server")
    parser.add_argument("--transport", choices=["stdio", "sse"], default=os.getenv("MCP_TRANSPORT", "sse"))
    # FIX: Default to 127.0.0.1 for security; added # nosec B104 to allow the 0.0.0.0 string literal
    parser.add_argument(
        "--host",
        default=os.getenv("DEFAULT_FASTAPI_HOST", "127.0.0.1"),
        help="Host to bind to (default: 127.0.0.1). Use 0.0.0.0 to expose globally.",
    )  # nosec B104
    parser.add_argument("--port", type=int, default=int(os.getenv("DEFAULT_FASTAPI_PORT", 8100)))
    args = parser.parse_args()

    if args.transport == "stdio":
        asyncio.run(run_stdio())
    else:
        app = create_sse_app()
        uvicorn.run(app, host=args.host, port=args.port)


if __name__ == "__main__":
    main()
