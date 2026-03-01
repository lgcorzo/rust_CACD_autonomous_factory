"""Tests for AutonomousMission workflow."""

import pytest
from unittest.mock import MagicMock, patch, AsyncMock
from autogen_team.application.workflows.autonomous_mission import (
    plan,
    fan_out_tasks,
    aggregate_and_review,
    MissionInput,
    TaskInput
)

@pytest.fixture
def mock_context():
    context = MagicMock()
    context.log = MagicMock()
    return context

@pytest.fixture
def mission_input():
    return MissionInput(goal="Test goal", repository_path="/test/path")

@pytest.mark.asyncio
async def test_plan_step(mock_context, mission_input):
    with patch("autogen_team.application.workflows.autonomous_mission.PlannerAgent") as mock_planner_cls:
        mock_planner = AsyncMock()
        mock_planner.create_plan.return_value = {"tasks": []}
        mock_planner_cls.return_value = mock_planner
        
        # Use .fn to call the decorated task
        result = await plan.fn(mission_input, mock_context)
        
        assert "plan" in result
        mock_planner.create_plan.assert_called_once_with(
            goal=mission_input.goal,
            repository_path=mission_input.repository_path
        )

@pytest.mark.asyncio
async def test_fan_out_tasks_step(mock_context, mission_input):
    # Mock context.task_output(plan)
    mock_context.task_output.return_value = {
        "plan": {
            "tasks": [
                {"id": "task1", "description": "desc1"},
                {"id": "task2", "description": "desc2"}
            ]
        }
    }
    
    with patch("autogen_team.application.workflows.autonomous_mission.develop_task_workflow") as mock_workflow:
        mock_workflow.create_bulk_run_item = MagicMock(side_effect=lambda input, key: MagicMock())
        mock_workflow.aio_run_many = AsyncMock(return_value=[{"status": "ok"}, {"status": "ok"}])
        
        # Use .fn
        result = await fan_out_tasks.fn(mission_input, mock_context)
        
        assert len(result["results"]) == 2
        assert mock_workflow.aio_run_many.called
        assert mock_workflow.create_bulk_run_item.call_count == 2

@pytest.mark.asyncio
async def test_aggregate_and_review_step(mock_context, mission_input):
    mock_context.task_output.return_value = {"results": [{"file_changes": ["file1.py"]}]}
    
    with patch("autogen_team.application.workflows.autonomous_mission.TesterAgent") as mock_tester_cls, \
         patch("autogen_team.application.workflows.autonomous_mission.ReviewerAgent") as mock_reviewer_cls:
        
        mock_tester = AsyncMock()
        mock_tester.run_tests.return_value = {"status": "passed", "report": "All tests passed"}
        mock_tester_cls.return_value = mock_tester
        
        mock_reviewer = AsyncMock()
        mock_review_result = MagicMock()
        mock_review_result.approved = True
        mock_review_result.comments = ["Good changes"]
        mock_reviewer.review_changes.return_value = mock_review_result
        mock_reviewer_cls.return_value = mock_reviewer
        
        # Use .fn
        result = await aggregate_and_review.fn(mission_input, mock_context)
        
        assert result.status == "success"
        assert "Mission success" in result.summary
        mock_tester.run_tests.assert_called_once()
        mock_reviewer.review_changes.assert_called_once()
