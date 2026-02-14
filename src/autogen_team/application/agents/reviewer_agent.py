from typing import Dict, Any, List
from autogen_team.infrastructure.messaging.a2a_protocol import ReviewResult


class ReviewerAgent:
    """
    Agent responsible for reviewing code changes.
    Currently mocks the MCP tool call.
    """

    def __init__(self):
        pass

    async def review_changes(self, mission_id: str, file_changes: List[str]) -> ReviewResult:
        """
        Calls the `security_review` tool via MCP (mocked for now).
        """
        print(f"[ReviewerAgent] Reviewing changes for mission: {mission_id}")

        # TODO: Replace with actual MCP tool call
        # result = await mcp_client.call_tool("security_review", {"files": file_changes})

        return ReviewResult(
            mission_id=mission_id,
            approved=True,
            comments=["Code looks good.", "No security issues found."],
            suggested_changes=None,
        )
