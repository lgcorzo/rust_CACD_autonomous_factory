from typing import Dict, Any

from autogen_team.infrastructure.client.mcp_client import MCPClient


class PlannerAgent:
    """
    Agent responsible for decomposing a high-level goal into a detailed plan.
    Uses the MCP 'plan_mission' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def create_plan(self, goal: str, repository_path: str) -> Dict[str, Any]:
        """
        Calls the `plan_mission` tool via MCP.
        """
        print(f"[PlannerAgent] Creating plan for goal: {goal}")

        try:
            await self.client.connect()
            result = await self.client.call_tool(
                "plan_mission", 
                {"goal": goal}
            )
            return result
        finally:
            await self.client.disconnect()
