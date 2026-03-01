"""Tests for agents."""

import pytest
from unittest.mock import MagicMock, patch, AsyncMock
from autogen_team.application.agents.coder_agent import CoderAgent
from autogen_team.application.agents.planner_agent import PlannerAgent
from autogen_team.application.agents.reviewer_agent import ReviewerAgent
from autogen_team.application.agents.tester_agent import TesterAgent

@pytest.fixture
def mock_mcp_client(mocker):
    mock_instance = mocker.MagicMock()
    mock_instance.connect = mocker.AsyncMock()
    mock_instance.disconnect = mocker.AsyncMock()
    mock_instance.call_tool = mocker.AsyncMock()
    mocker.patch("autogen_team.application.agents.coder_agent.MCPClient", return_value=mock_instance)
    mocker.patch("autogen_team.application.agents.planner_agent.MCPClient", return_value=mock_instance)
    mocker.patch("autogen_team.application.agents.reviewer_agent.MCPClient", return_value=mock_instance)
    mocker.patch("autogen_team.application.agents.tester_agent.MCPClient", return_value=mock_instance)
    return mock_instance

@pytest.mark.asyncio
async def test_coder_agent_execute_task(mock_mcp_client):
    agent = CoderAgent()
    mock_mcp_client.call_tool.return_value = {"status": "success"}
    
    result = await agent.execute_task({"id": "task1"})
    
    assert result == {"status": "success"}
    mock_mcp_client.connect.assert_called_once()
    mock_mcp_client.call_tool.assert_called_with("execute_code", {"task": {"id": "task1"}})
    mock_mcp_client.disconnect.assert_called_once()

@pytest.mark.asyncio
async def test_planner_agent_create_plan(mock_mcp_client):
    agent = PlannerAgent()
    mock_mcp_client.call_tool.return_value = {"tasks": []}
    
    result = await agent.create_plan("goal", "/path")
    
    assert result == {"tasks": []}
    mock_mcp_client.call_tool.assert_called_with("plan_mission", {"goal": "goal"})

@pytest.mark.asyncio
async def test_reviewer_agent_review_changes(mock_mcp_client):
    agent = ReviewerAgent()
    mock_mcp_client.call_tool.return_value = {"status": "approved", "analysis": "Looks good"}
    
    result = await agent.review_changes("mission1", ["line1", "line2"])
    
    assert result.approved is True
    assert result.comments == ["Looks good"]
    mock_mcp_client.call_tool.assert_called_with("security_review", {"diff": "line1\nline2"})

@pytest.mark.asyncio
async def test_tester_agent_run_tests(mock_mcp_client):
    agent = TesterAgent()
    mock_mcp_client.call_tool.return_value = {"status": "passed"}
    
    result = await agent.run_tests()
    
    assert result == {"status": "passed"}
    mock_mcp_client.call_tool.assert_called_with("run_tests", {})
