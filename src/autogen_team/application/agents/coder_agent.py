from typing import Dict, Any, List

from autogen_team.infrastructure.client.mcp_client import MCPClient


class CoderAgent:
    """
    Agent responsible for executing coding tasks.
    Uses the MCP 'execute_code' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def execute_task(self, task: Dict[str, Any]) -> Dict[str, Any]:
        """
        Calls the `execute_code` tool via MCP.
        """
        print(f"[CoderAgent] Executing task: {task.get('description', 'unknown')}")

        try:
            await self.client.connect()
            result = await self.client.call_tool(
                "execute_code", 
                {"task": task, "workspace_path": "."}
            )
            return result
        finally:
            await self.client.disconnect()
