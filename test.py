import asyncio
from mcp import ClientSession, StdioServerParameters
from mcp.client.sse import sse_client

async def test_mcp():
    async with sse_client("http://localhost:8400/sse/") as streams:
        async with ClientSession(streams[0], streams[1]) as session:
            await session.initialize()
            print("Connected!")
            
            tools = await session.list_tools()
            print("\nAvailable tools:")
            for tool in tools.tools:
                print(f"- {tool.name}: {tool.description}")

if __name__ == "__main__":
    asyncio.run(test_mcp())
