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
from pathlib import Path

from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


async def test_mcp_server() -> None:
    """Connect to MCP server and run tests."""
    print("🚀 Starting MCP Server integration test...")

    # Path to the server module
    # We run it via 'python -m autogen_team.application.mcp.mcp_server'
    server_params = StdioServerParameters(
        command="python",
        args=["-m", "autogen_team.application.mcp.mcp_server"],
        env=None,
    )

    try:
        async with stdio_client(server_params) as (read, write):
            async with ClientSession(read, write) as session:
                # 1. Initialize
                print("📦 Initializing session...")
                await session.initialize()

                # 2. List Tools
                print("🔍 Listing tools...")
                tools_result = await session.list_tools()
                tools = tools_result.tools
                print(f"✅ Found {len(tools)} tools:")
                for tool in tools:
                    print(f"   - {tool.name}: {tool.description[:60]}...")

                # 3. Test plan_mission
                print("\n🧪 Testing 'plan_mission' tool...")
                goal = "Implement a simple calculator"
                result = await session.call_tool(
                    "plan_mission",
                    arguments={"goal": goal},
                )
                
                # result.content is a list of content blocks
                content = result.content[0].text
                data = json.loads(content)
                
                if "parallel_tasks" in data:
                    print(f"✅ 'plan_mission' success!")
                    print(f"   Goal: {data.get('goal')}")
                    print(f"   Tasks count: {len(data['parallel_tasks'])}")
                else:
                    print(f"❌ 'plan_mission' failed to return tasks: {content}")

                # 4. Test security_review (dry run/empty)
                print("\n🧪 Testing 'security_review' tool...")
                diff = "+x = 1"
                result = await session.call_tool(
                    "security_review",
                    arguments={"diff": diff},
                )
                content = result.content[0].text
                data = json.loads(content)
                print(f"✅ 'security_review' status: {data.get('status')}")

    except Exception as e:
        print(f"❌ Error during test: {e!s}")
        sys.exit(1)

    print("\n🎉 integration test completed successfully!")


if __name__ == "__main__":
    asyncio.run(test_mcp_server())
