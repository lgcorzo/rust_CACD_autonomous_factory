import pytest
from unittest.mock import AsyncMock, MagicMock, patch
from autogen_team.application.workflows.autonomous_mission import (
    MissionInput,
    TaskInput,
    MissionOutput,
    execute_coding_task,
    plan,
    fan_out_tasks,
    aggregate_and_review,
    document_mission,
)


@pytest.fixture
def mock_context() -> MagicMock:
    ctx = MagicMock()
    ctx.log = MagicMock()
    ctx.task_output = MagicMock()
    return ctx


@pytest.mark.asyncio
async def test_execute_coding_task(mock_context: MagicMock) -> None:
    task_input = TaskInput(task_id="t1", description="desc", relevant_files=["f1"])
    with patch("autogen_team.application.workflows.autonomous_mission.CoderAgent") as MockCoder:
        mock_coder = MockCoder.return_value
        mock_coder.execute_task = AsyncMock(return_value={"status": "done"})
        result = await execute_coding_task.fn(task_input, mock_context)  # type: ignore[call-arg]

    assert result == {"status": "done"}
    mock_coder.execute_task.assert_called_once()


@pytest.mark.asyncio
async def test_plan(mock_context: MagicMock) -> None:
    mission_input = MissionInput(goal="test goal", repository_path="/repo")
    with patch("autogen_team.application.workflows.autonomous_mission.PlannerAgent") as MockPlanner:
        mock_planner = MockPlanner.return_value
        mock_planner.create_plan = AsyncMock(return_value={"tasks": []})
        result = await plan.fn(mission_input, mock_context)  # type: ignore[call-arg]

    assert result == {"plan": {"tasks": []}}
    mock_planner.create_plan.assert_called_once()


@pytest.mark.asyncio
async def test_fan_out_tasks(mock_context: MagicMock) -> None:
    mission_input = MissionInput(goal="goal", repository_path="/p")
    mock_context.task_output.return_value = {"plan": {"tasks": [{"id": "1", "description": "d1"}]}}

    with patch(
        "autogen_team.application.workflows.autonomous_mission.develop_task_workflow"
    ) as mock_wf:
        mock_wf.aio_run_many = AsyncMock(return_value=[{"task": "1", "status": "ok"}])
        mock_wf.create_bulk_run_item = MagicMock(return_value="bulk_item")

        result = await fan_out_tasks.fn(mission_input, mock_context)  # type: ignore[call-arg]

    assert result == {"results": [{"task": "1", "status": "ok"}]}
    mock_wf.aio_run_many.assert_called_once()


@pytest.mark.asyncio
async def test_aggregate_and_review(mock_context: MagicMock) -> None:
    mission_input = MissionInput(goal="goal", repository_path="/p")
    mock_context.task_output.return_value = {"results": [{"file_changes": ["diff1"]}]}

    with (
        patch("autogen_team.application.workflows.autonomous_mission.TesterAgent") as MockTester,
        patch(
            "autogen_team.application.workflows.autonomous_mission.ReviewerAgent"
        ) as MockReviewer,
    ):
        MockTester.return_value.run_tests = AsyncMock(
            return_value={"status": "passed", "report": "all ok"}
        )
        MockReviewer.return_value.review_changes = AsyncMock(
            return_value=MagicMock(approved=True, comments=["looks good"])
        )

        result = await aggregate_and_review.fn(mission_input, mock_context)  # type: ignore[call-arg]

    assert result.status == "success"
    assert "looks good" in result.summary


@pytest.mark.asyncio
async def test_document_mission(mock_context: MagicMock) -> None:
    mission_input = MissionInput(goal="goal", repository_path="/p")
    mock_context.task_output.side_effect = lambda t: {
        aggregate_and_review: MissionOutput(status="success", summary="Review done"),
        plan: {"plan": {"tasks": [{"id": "t1"}]}},
        fan_out_tasks: {"results": [{"file_changes": ["diff1"]}]},
    }.get(t)

    with patch(
        "autogen_team.application.workflows.autonomous_mission.DocumentationAgent"
    ) as MockDocAgent:
        MockDocAgent.return_value.generate_docs = AsyncMock(
            return_value={"summary": "Docs done", "diagrams": {}}
        )

        result = await document_mission.fn(mission_input, mock_context)  # type: ignore[call-arg]

    assert result.status == "success"
    assert "Review done" in result.summary
    assert "Docs done" in result.summary
