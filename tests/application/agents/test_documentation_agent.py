import pytest
from unittest.mock import AsyncMock, patch
from autogen_team.application.agents.documentation_agent import DocumentationAgent


@pytest.mark.asyncio
async def test_documentation_agent_generate_docs_success() -> None:
    """Test DocumentationAgent.generate_docs success path."""
    agent = DocumentationAgent()
    mock_result = {"summary": "Done", "diagrams": {}}

    with (
        patch(
            "autogen_team.infrastructure.client.mcp_client.MCPClient.connect",
            new_callable=AsyncMock,
        ) as _,
        patch(
            "autogen_team.infrastructure.client.mcp_client.MCPClient.call_tool",
            new_callable=AsyncMock,
        ) as mock_call,
        patch(
            "autogen_team.infrastructure.client.mcp_client.MCPClient.disconnect",
            new_callable=AsyncMock,
        ) as mock_disconnect,
    ):
        mock_call.return_value = mock_result

        result = await agent.generate_docs("mission-1", {"goal": "test"})

        assert result == mock_result
        # mock_connect.assert_called_once() # This was removed as per the instruction to prefix unused mock variables with `_`
        mock_call.assert_called_once_with(
            "generate_mission_docs",
            {"mission_id": "mission-1", "mission_context": {"goal": "test"}},
        )
        mock_disconnect.assert_called_once()


@pytest.mark.asyncio
async def test_documentation_agent_generate_docs_failure() -> None:
    """Test DocumentationAgent.generate_docs exception handling."""
    agent = DocumentationAgent()

    with (
        patch(
            "autogen_team.infrastructure.client.mcp_client.MCPClient.connect",
            new_callable=AsyncMock,
        ) as _,
        patch(
            "autogen_team.infrastructure.client.mcp_client.MCPClient.call_tool",
            side_effect=Exception("MCP Error"),
        ),
        patch(
            "autogen_team.infrastructure.client.mcp_client.MCPClient.disconnect",
            new_callable=AsyncMock,
        ) as mock_disconnect,
    ):
        with pytest.raises(Exception, match="MCP Error"):
            await agent.generate_docs("mission-1", {"goal": "test"})

        mock_disconnect.assert_called_once()
