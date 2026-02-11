"""Tests for plan_mission tool."""

from __future__ import annotations

import json
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from autogen_team.application.mcp.tools.plan_mission import plan_mission


@pytest.mark.asyncio
async def test_plan_mission_valid_goal(sample_goal: str) -> None:
    """Test plan_mission with a valid goal returns a DAG."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = json.dumps(
        {
            "goal": sample_goal,
            "parallel_tasks": [
                {
                    "id": "task_1",
                    "name": "Setup project",
                    "description": "Initialize project structure",
                    "depends_on": [],
                },
                {
                    "id": "task_2",
                    "name": "Create models",
                    "description": "Define data models",
                    "depends_on": ["task_1"],
                },
            ],
        }
    )

    with patch("autogen_team.application.mcp.tools.plan_mission.litellm") as mock_litellm:
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        result = await plan_mission(sample_goal)

    assert "parallel_tasks" in result
    assert len(result["parallel_tasks"]) == 2
    assert result["parallel_tasks"][0]["id"] == "task_1"
    assert result["goal"] == sample_goal


@pytest.mark.asyncio
async def test_plan_mission_empty_goal() -> None:
    """Test plan_mission with empty goal returns error."""
    result = await plan_mission("")
    assert result["error"] == "Empty goal provided"
    assert result["parallel_tasks"] == []


@pytest.mark.asyncio
async def test_plan_mission_malformed_response(sample_goal: str) -> None:
    """Test plan_mission handles malformed LLM response."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = "not valid json {{"

    with patch("autogen_team.application.mcp.tools.plan_mission.litellm") as mock_litellm:
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        result = await plan_mission(sample_goal)

    assert "error" in result
    assert "Failed to parse" in result["error"]
    assert result["parallel_tasks"] == []
