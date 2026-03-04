from typing import Any, Dict

from autogen_team.infrastructure.client.mcp_client import MCPClient


class DocumentationAgent:
    """
    Agent responsible for generating mission documentation and diagrams.
    Uses the MCP 'generate_mission_docs' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def generate_docs(
        self, mission_id: str, mission_context: Dict[str, Any]
    ) -> Dict[str, Any]:
        """
        Calls the `generate_mission_docs` tool via MCP.
        """
        print(f"[DocumentationAgent] Generating documentation for mission: {mission_id}")

        try:
            await self.client.connect()
            result = await self.client.call_tool(
                "generate_mission_docs",
                {"mission_id": mission_id, "mission_context": mission_context},
            )
            import typing

            return typing.cast(Dict[str, Any], result)
        finally:
            await self.client.disconnect()
