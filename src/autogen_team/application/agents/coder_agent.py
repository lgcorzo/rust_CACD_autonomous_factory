from typing import Dict, Any
from typing import cast

from autogen_team.infrastructure.client.mcp_client import MCPClient


class CoderAgent:
    """
    Agent responsible for executing coding tasks.
    Uses the MCP 'execute_code' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def execute_task(self, task: Dict[str, Any]) -> Dict[str, Any]:
        print(f"[CoderAgent] Executing task: {task.get('id', 'unknown')}")

        try:
            await self.client.connect()
            result = await self.client.call_tool("execute_code", {"task": task})
            return cast(Dict[str, Any], result)
        finally:
            await self.client.disconnect()
