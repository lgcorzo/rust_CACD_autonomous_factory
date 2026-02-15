"""Hatchet Workflow DSL for Autonomous Missions.

Orchestrates the autonomous mission lifecycle:
    Plan → Fan-Out (parallel coding) → Aggregate & Review

Uses the Hatchet V1 SDK with ``aio_run_many`` for true parallel
child-workflow fan-out instead of sequential task execution.
"""

from typing import Any, Dict, List

from hatchet_sdk import Context
from pydantic import BaseModel

from autogen_team.application.agents.coder_agent import CoderAgent
from autogen_team.application.agents.planner_agent import PlannerAgent
from autogen_team.application.agents.reviewer_agent import ReviewerAgent
from autogen_team.application.agents.tester_agent import TesterAgent
from autogen_team.infrastructure.services.hatchet_service import HatchetService

# ---------------------------------------------------------------------------
# Hatchet client
# ---------------------------------------------------------------------------
hatchet_service = HatchetService()
hatchet = hatchet_service.client

# ---------------------------------------------------------------------------
# Input / Output models
# ---------------------------------------------------------------------------


class MissionInput(BaseModel):
    """Input for the top-level autonomous-mission workflow."""

    goal: str
    repository_path: str


class TaskInput(BaseModel):
    """Input for a single child coding-task workflow."""

    task_id: str
    description: str
    relevant_files: List[str] = []
    constraints: str | None = None


class MissionOutput(BaseModel):
    """Final output of the autonomous-mission workflow."""

    status: str
    pull_request_url: str = ""
    summary: str


# ===================================================================
# Child workflow – executed once per coding task (fan-out target)
# ===================================================================
develop_task_workflow = hatchet.workflow(
    name="DevelopTaskWorkflow",
    input_validator=TaskInput,
)


@develop_task_workflow.task(execution_timeout="15m")
async def execute_coding_task(
    task_input: TaskInput, context: Context
) -> Dict[str, Any]:
    """Run the Coder Agent on a single task inside a child workflow."""
    context.log(f"Child workflow: executing task {task_input.task_id}")

    coder = CoderAgent()
    result = await coder.execute_task(
        {
            "id": task_input.task_id,
            "description": task_input.description,
            "relevant_files": task_input.relevant_files,
            "constraints": task_input.constraints,
        }
    )
    return result


# ===================================================================
# Parent workflow – orchestrates Plan → Fan-Out → Review
# ===================================================================
autonomous_mission_workflow = hatchet.workflow(
    name="AutonomousMissionWorkflow",
    on_events=["autonomous_mission"],
    input_validator=MissionInput,
)


@autonomous_mission_workflow.task(execution_timeout="5m")
async def plan(
    mission_input: MissionInput, context: Context
) -> Dict[str, Any]:
    """Step 1: Planner Agent analyses the goal and creates a task DAG."""
    context.log(f"Planning mission: {mission_input.goal}")

    planner = PlannerAgent()
    mission_plan = await planner.create_plan(
        goal=mission_input.goal,
        repository_path=mission_input.repository_path,
    )
    return {"plan": mission_plan}


@autonomous_mission_workflow.task(parents=[plan], execution_timeout="30m")
async def fan_out_tasks(
    mission_input: MissionInput, context: Context
) -> Dict[str, Any]:
    """Step 2: Spawn parallel child workflows for each coding task.

    Uses ``develop_task_workflow.aio_run_many`` for true parallel
    fan-out execution across the Hatchet worker pool.
    """
    plan_data = context.task_output(plan)["plan"]
    tasks: list[dict[str, Any]] = plan_data["tasks"]
    context.log(f"Fanning out {len(tasks)} tasks in parallel")

    # Build bulk-run items for each task
    bulk_items = [
        develop_task_workflow.create_bulk_run_item(
            input=TaskInput(
                task_id=t["id"],
                description=t.get("description", ""),
                relevant_files=t.get("relevant_files", []),
                constraints=t.get("constraints"),
            ),
            key=t["id"],
        )
        for t in tasks
    ]

    # Fan-out: launch all child workflows in parallel and wait for results
    results: list[dict[str, Any]] = await develop_task_workflow.aio_run_many(
        workflows=bulk_items,
    )

    return {"results": results}


@autonomous_mission_workflow.task(
    parents=[fan_out_tasks], execution_timeout="15m"
)
async def aggregate_and_review(
    mission_input: MissionInput, context: Context
) -> MissionOutput:
    """Step 3: Aggregate child results, run tests, and perform security review."""
    fan_out_output = context.task_output(fan_out_tasks)
    task_results = fan_out_output["results"]
    context.log("Aggregating results and reviewing...")

    # --- Testing ---
    tester = TesterAgent()
    test_results = await tester.run_tests()

    # --- Security Review ---
    reviewer = ReviewerAgent()
    all_file_changes: list[str] = []
    for res in task_results:
        all_file_changes.extend(res.get("file_changes", []))

    review_result = await reviewer.review_changes(
        mission_id="mission-auto",
        file_changes=all_file_changes,
    )

    status = (
        "success"
        if test_results.get("status") == "passed" and review_result.approved
        else "failed"
    )

    summary = (
        f"Mission {status}. "
        f"Tests: {test_results.get('report', 'N/A')} "
        f"Review: {', '.join(review_result.comments)}"
    )

    return MissionOutput(
        status=status,
        pull_request_url="https://github.com/org/repo/pull/123",
        summary=summary,
    )
