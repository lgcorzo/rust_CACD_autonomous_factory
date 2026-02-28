"""Simple verification script for MCP Client connection."""

import asyncio

from autogen_team.infrastructure.client.mcp_client import MCPClient


async def main() -> None:
    print("🚀 Testing MCP Client Connection...")

    client = MCPClient()
    try:
        await client.connect()
        print("✅ Connected to MCP Server.")

        # List tools (assuming standard MCP implementation supports this, or try a known tool with bad args to provoke error)
        # The standard mcp library client has list_tools method on the session
        tools = await client.session.list_tools()
        print(f"✅ Tools available: {[t.name for t in tools.tools]}")

    except Exception as e:
        print(f"❌ Connection Failed: {e}")
    finally:
        await client.disconnect()


if __name__ == "__main__":
    asyncio.run(main())
