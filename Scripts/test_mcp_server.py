"""Integration test script for the MCP server.

This script starts the MCP server as a subprocess using stdio transport,
connects as a client, and verifies tool listing and execution.

Usage:
    poetry run python Scripts/test_mcp_server.py
"""

from __future__ import annotations

import asyncio
import json
import sys

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


async def test_mcp_server() -> None:
    """Connect to MCP server and run tests."""
    print("🚀 Starting MCP Server integration test...")

    # Path to the server module
    # We run it via 'python -m autogen_team.application.mcp.mcp_server'
    # We explicitly pass the prompts config path
    import os

    env = os.environ.copy()
    env["MCP_PROMPTS_PATH"] = "confs/mcp_prompts.yaml"

    server_params = StdioServerParameters(
        command="python",
        args=["-m", "autogen_team.application.mcp.mcp_server"],
        env=env,
    )

    try:
        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                # 1. Initialize
                print("📦 Initializing session...", flush=True)
                await session.initialize()

                # 2. List Tools
                print("🔍 Listing tools...", flush=True)
                tools_result = await session.list_tools()
                tools = tools_result.tools
                print(f"✅ Found {len(tools)} tools:", flush=True)
                for tool in tools:
                    desc: str = tool.description or ""
                    print(f"   - {tool.name}: {desc[:60]}...", flush=True)

                # 3. Test plan_mission
                print("\n🧪 Testing 'plan_mission' tool...", flush=True)
                goal = "Implement a simple calculator"
                try:
                    result = await asyncio.wait_for(
                        session.call_tool(
                            "plan_mission",
                            arguments={"goal": goal},
                        ),
                        timeout=30.0,
                    )
                except asyncio.TimeoutError:
                    print("❌ 'plan_mission' timed out after 30s", flush=True)
                    raise

                # result.content is a list of content blocks
                content = result.content[0].text
                data = json.loads(content)

                if "parallel_tasks" in data:
                    print("✅ 'plan_mission' success!", flush=True)
                    print(f"   Goal: {data.get('goal')}", flush=True)
                    print(f"   Tasks count: {len(data['parallel_tasks'])}", flush=True)
                else:
                    print(f"❌ 'plan_mission' failed to return tasks: {content}", flush=True)

                # 4. Test security_review (dry run/empty)
                print("\n🧪 Testing 'security_review' tool...", flush=True)
                diff = "+x = 1"
                try:
                    result = await asyncio.wait_for(
                        session.call_tool(
                            "security_review",
                            arguments={"diff": diff},
                        ),
                        timeout=30.0,
                    )
                except asyncio.TimeoutError:
                    print("❌ 'security_review' timed out after 30s", flush=True)
                    raise
                content = result.content[0].text
                data = json.loads(content)
                print(f"✅ 'security_review' status: {data.get('status')}", flush=True)

    except Exception:
        import traceback

        print("\n❌ Error during test:", flush=True)
        traceback.print_exc()
        sys.exit(1)

    print("\n🎉 integration test completed successfully!", flush=True)


if __name__ == "__main__":
    asyncio.run(test_mcp_server())
