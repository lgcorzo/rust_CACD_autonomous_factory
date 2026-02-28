"""MCP Server — Model Context Protocol server bootstrap.

Registers all 6 tools and exposes them via the MCP protocol.
Supports both stdio and SSE transports.

Usage:
    python -m autogen_team.application.mcp.mcp_server
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

# Crucial imports for solving the mypy [arg-type] and [no-untyped-def] errors
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
    """Create and configure the MCP server with all tools registered.

    Returns:
        Configured MCP Server instance.
    """
    return _server


@_server.list_tools()  # type: ignore[untyped-decorator]
async def handle_list_tools() -> T.List[Tool]:
    """List all available MCP tools."""
    return [
        Tool(
            name="plan_mission",
            description=(
                "Decomposes a high-level goal into a task DAG using LiteLLM (Gemini Pro 2.5). "
                "Input: goal string. Output: JSON DAG with parallel_tasks array."
            ),
            inputSchema={
                "type": "object",
                "properties": {
                    "goal": {
                        "type": "string",
                        "description": "The high-level goal to decompose into tasks.",
                    },
                },
                "required": ["goal"],
            },
        ),
        Tool(
            name="execute_code",
            description=(
                "Generates code changes and validates in sandbox. "
                "Input: task dict + workspace path. Output: file diffs."
            ),
            inputSchema={
                "type": "object",
                "properties": {
                    "task": {
                        "type": "object",
                        "description": "Task dict with id, name, description.",
                    },
                    "workspace_path": {
                        "type": "string",
                        "description": "Path to the workspace root.",
                    },
                },
                "required": ["task", "workspace_path"],
            },
        ),
        Tool(
            name="run_tests",
            description=(
                "Runs pytest in isolated sandbox. "
                "Input: changes dict + workspace path. Output: test results."
            ),
            inputSchema={
                "type": "object",
                "properties": {
                    "changes": {
                        "type": "object",
                        "description": "Dict with files_changed list.",
                    },
                    "workspace_path": {
                        "type": "string",
                        "description": "Original workspace path.",
                        "default": "",
                    },
                    "timeout": {
                        "type": "integer",
                        "description": "Max execution time in seconds.",
                        "default": 300,
                    },
                },
                "required": ["changes"],
            },
        ),
        Tool(
            name="security_review",
            description=(
                "Analyzes code diffs against OWASP patterns and R2R RAG. "
                "Input: diff string. Output: approved/rejected with findings."
            ),
            inputSchema={
                "type": "object",
                "properties": {
                    "diff": {
                        "type": "string",
                        "description": "The code diff string to review.",
                    },
                },
                "required": ["diff"],
            },
        ),
        Tool(
            name="retrieve_context",
            description=(
                "Queries R2R RAG for relevant codebase patterns via semantic search. "
                "Input: query + collection name. Output: matching documents + graph context."
            ),
            inputSchema={
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search query string.",
                    },
                    "collection_name": {
                        "type": "string",
                        "description": "R2R collection name.",
                        "default": "default",
                    },
                },
                "required": ["query"],
            },
        ),
        Tool(
            name="index_code",
            description=(
                "Indexes code files into R2R knowledge graph. "
                "Input: file_path + content + metadata. Output: ingestion confirmation."
            ),
            inputSchema={
                "type": "object",
                "properties": {
                    "file_path": {
                        "type": "string",
                        "description": "Path of the file being indexed.",
                    },
                    "content": {
                        "type": "string",
                        "description": "Full content of the file.",
                    },
                    "metadata": {
                        "type": "object",
                        "description": "Optional metadata dict.",
                        "default": {},
                    },
                },
                "required": ["file_path", "content"],
            },
        ),
    ]


@_server.call_tool()  # type: ignore[untyped-decorator]
async def handle_call_tool(name: str, arguments: T.Dict[str, T.Any] | None) -> T.List[TextContent]:
    """Dispatch tool calls to the appropriate handler."""
    args = arguments or {}

    try:
        if name == "plan_mission":
            result = await plan_mission(goal=args.get("goal", ""))
        elif name == "execute_code":
            result = await execute_code(
                task=args.get("task", {}),
                workspace_path=args.get("workspace_path", ""),
            )
        elif name == "run_tests":
            result = await run_tests(
                changes=args.get("changes", {}),
                workspace_path=args.get("workspace_path", ""),
                timeout=args.get("timeout", 300),
            )
        elif name == "security_review":
            result = await security_review(diff=args.get("diff", ""))
        elif name == "retrieve_context":
            result = await retrieve_context(
                query=args.get("query", ""),
                collection_name=args.get("collection_name", "default"),
            )
        elif name == "index_code":
            result = await index_code(
                file_path=args.get("file_path", ""),
                content=args.get("content", ""),
                metadata=args.get("metadata"),
            )
        else:
            result = {"error": f"Unknown tool: {name}"}
    except Exception as e:
        import traceback

        result = {
            "error": str(e),
            "tool": name,
            "status": "failed",
            "traceback": traceback.format_exc(),
        }

    return [TextContent(type="text", text=json.dumps(result, indent=2))]


async def run_stdio() -> None:
    """Run the MCP server using stdio transport."""
    async with stdio_server() as (read_stream, write_stream):
        await _server.run(
            read_stream,
            write_stream,
            _server.create_initialization_options(),
        )


def create_sse_app() -> Starlette:
    """Create a Starlette app for SSE transport."""
    sse = SseServerTransport("/messages/")

    # FIXED: Added Scope, Receive, and Send type hints to satisfy mypy
    async def _sse_asgi(scope: Scope, receive: Receive, send: Send) -> None:
        """Raw ASGI handler for the SSE endpoint."""
        async with sse.connect_sse(scope, receive, send) as (
            read_stream,
            write_stream,
        ):
            await _server.run(
                read_stream,
                write_stream,
                _server.create_initialization_options(),
            )

    async def health_check(request: Request) -> Response:
        return Response(json.dumps({"status": "healthy"}), media_type="application/json")

    routes = [
        # FIXED: Now matches the expected ASGI application signature
        Mount("/sse", app=_sse_asgi),
        Mount("/messages/", app=sse.handle_post_message),
        Route("/health", endpoint=health_check, methods=["GET"]),
    ]
    return Starlette(routes=routes)


def main() -> None:
    """Run the MCP server."""
    parser = argparse.ArgumentParser(description="MCP Server")
    parser.add_argument(
        "--transport", choices=["stdio", "sse"], default=os.getenv("MCP_TRANSPORT", "sse")
    )
    parser.add_argument("--host", default=os.getenv("DEFAULT_FASTAPI_HOST", "0.0.0.0"))
    parser.add_argument("--port", type=int, default=int(os.getenv("DEFAULT_FASTAPI_PORT", 8100)))
    args = parser.parse_args()

    if args.transport == "stdio":
        asyncio.run(run_stdio())
    else:
        app = create_sse_app()
        uvicorn.run(app, host=args.host, port=args.port)


if __name__ == "__main__":
    main()
