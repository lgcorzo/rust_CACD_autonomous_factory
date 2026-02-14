from typing import Dict, Any, cast

from autogen_team.infrastructure.client.mcp_client import MCPClient


class TesterAgent:
    """
    Agent responsible for running tests.
    Uses the MCP 'run_tests' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def run_tests(self) -> Dict[str, Any]:
        print("[TesterAgent] Running tests...")

        try:
            await self.client.connect()
            result = await self.client.call_tool("run_tests", {})
            return cast(Dict[str, Any], result)
        finally:
            await self.client.disconnect()
