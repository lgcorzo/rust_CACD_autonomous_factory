from typing import Dict, Any, List

from autogen_team.infrastructure.client.mcp_client import MCPClient


class TesterAgent:
    """
    Agent responsible for running tests.
    Uses the MCP 'run_tests' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def run_tests(self, test_scope: str = "all") -> Dict[str, Any]:
        """
        Calls the `run_tests` tool via MCP.
        """
        print(f"[TesterAgent] Running tests for scope: {test_scope}")

        try:
            await self.client.connect()
            result = await self.client.call_tool(
                "run_tests", 
                {"workspace_path": "."}
            )
            return result
        finally:
            await self.client.disconnect()
