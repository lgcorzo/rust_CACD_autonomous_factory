from typing import List

from autogen_team.infrastructure.client.mcp_client import MCPClient
from autogen_team.infrastructure.messaging.a2a_protocol import ReviewResult


class ReviewerAgent:
    """
    Agent responsible for reviewing code changes.
    Uses the MCP 'security_review' tool.
    """

    def __init__(self) -> None:
        self.client = MCPClient()

    async def review_changes(self, mission_id: str, file_changes: List[str]) -> ReviewResult:
        """
        Calls the `security_review` tool via MCP.
        """
        print(f"[ReviewerAgent] Reviewing changes for mission: {mission_id}")

        # Aggregate diffs if possible, or just pass filenames if that's what we have
        # Ideally file_changes should contain diffs.
        # For DA-7, we assume the tool handles string input or needs refactoring.
        # Here we just pass the list as a string for simplicity or update tool signature.
        diff_summary = "\n".join(file_changes)

        try:
            await self.client.connect()
            result = await self.client.call_tool("security_review", {"diff": diff_summary})

            # Map tool result to ReviewResult
            status = result.get("status", "unknown")
            approved = status == "approved"
            comments = [result.get("analysis", "No analysis provided")]

            return ReviewResult(
                mission_id=mission_id,
                approved=approved,
                comments=comments,
                suggested_changes=None,
            )
        finally:
            await self.client.disconnect()
