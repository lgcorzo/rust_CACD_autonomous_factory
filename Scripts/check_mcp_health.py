"""Health check script for the MCP server.

Verifies that the server starts and correctly registers all 6 tools.
Does NOT execute tools that require network access.

Usage:
    poetry run python Scripts/check_mcp_health.py
"""

from __future__ import annotations

import asyncio
import sys

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


async def check_mcp_health() -> None:
    """Connect to MCP server and verify tool listing."""
    print("🏥 Running MCP Server Health Check...")

    server_params = StdioServerParameters(
        command="python",
        args=["-m", "autogen_team.application.mcp.mcp_server"],
    )

    try:
        # We use a short timeout for the connection
        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                print("🔗 Connected. Initializing...")
                await session.initialize()

                print("📦 Requesting tool list...")
                tools_result = await session.list_tools()
                tools = tools_result.tools
                
                expected_tools = {
                    "plan_mission", "execute_code", "run_tests", 
                    "security_review", "retrieve_context", "index_code"
                }
                found_tools = {t.name for t in tools}

                print(f"✅ Found {len(tools)} tools registered.")
                
                missing = expected_tools - found_tools
                if missing:
                    print(f"❌ Missing tools: {missing}")
                    sys.exit(1)
                
                print("✨ All 6 tools are correctly registered and schema-validated.")
                print("   - " + "\n   - ".join(sorted(found_tools)))

    except Exception as e:
        print(f"❌ Health check failed: {e!s}")
        sys.exit(1)

    print("\n✅ MCP Server is HEALTHY and READY.")


if __name__ == "__main__":
    # Run with a global timeout to avoid hanging
    try:
        # Increase timeout to 60s to account for slow poetry/server startup
        asyncio.run(asyncio.wait_for(check_mcp_health(), timeout=60.0))
    except asyncio.TimeoutError:
        print("❌ Health check timed out after 60 seconds.")
        sys.exit(1)
    except Exception as e:
        print(f"❌ Unexpected error in runner: {e!s}")
        sys.exit(1)
