from typing import Dict, Any

from hatchet_sdk import Context
from pydantic import BaseModel
from autogen_team.infrastructure.services.hatchet_service import HatchetService
from autogen_team.application.agents.planner_agent import PlannerAgent
from autogen_team.application.agents.coder_agent import CoderAgent
from autogen_team.application.agents.tester_agent import TesterAgent
from autogen_team.application.agents.reviewer_agent import ReviewerAgent

# Initialize Hatchet via Service
hatchet_service = HatchetService()
hatchet = hatchet_service.client


# Input/Output Models
class MissionInput(BaseModel):
    goal: str
    repository_path: str


class MissionOutput(BaseModel):
    status: str
    pull_request_url: str = ""
    summary: str


# Create Workflow Object
autonomous_mission_workflow = hatchet.workflow(
    name="AutonomousMissionWorkflow",
    on_events=["mission:start"],
    input_validator=MissionInput,
)


@autonomous_mission_workflow.task(execution_timeout="5m")
async def plan(mission_input: MissionInput, context: Context) -> Dict[str, Any]:
    """
    Step 1: Planner Agent analyzes the goal and creates a plan.
    """
    context.log(f"Planning mission: {mission_input.goal}")

    planner = PlannerAgent()
    plan = await planner.create_plan(
        goal=mission_input.goal, repository_path=mission_input.repository_path
    )

    return {"plan": plan}


@autonomous_mission_workflow.task(parents=[plan], execution_timeout="30m")
async def fan_out_tasks(input: Any, context: Context) -> Dict[str, Any]:
    """
    Step 2: Fan-out coding tasks to Coder Agents.
    """
    # plan output is a dict {"plan": ...}
    plan_data = context.step_output("plan")["plan"]
    tasks = plan_data["tasks"]
    context.log(f"Fanning out {len(tasks)} tasks")

    results = []
    coder = CoderAgent()

    # In a real implementation, we would spawn child workflows or parallel steps.
    # For this DSL iteration, we simulate sequential execution of the fan-out for simplicity
    # or use Hatchet's dynamic fan-out if supported (spawn_workflow).

    for task in tasks:
        context.log(f"Executing task: {task['id']}")
        result = await coder.execute_task(task)
        results.append(result)

    return {"results": results}


@autonomous_mission_workflow.task(parents=[fan_out_tasks], execution_timeout="15m")
async def aggregate_and_review(input: Any, context: Context) -> MissionOutput:
    """
    Step 3: Aggregate results, run tests, and perform security review.
    """
    fan_out_output = context.step_output("fan_out_tasks")
    task_results = fan_out_output["results"]
    context.log("Aggregating results and reviewing...")

    tester = TesterAgent()
    test_results = await tester.run_tests()

    reviewer = ReviewerAgent()
    # Collect file changes from task results
    all_file_changes = []
    for res in task_results:
        all_file_changes.extend(res.get("file_changes", []))

    review_result = await reviewer.review_changes(
        mission_id="mission-mock", file_changes=all_file_changes
    )

    status = (
        "success" if test_results.get("status") == "passed" and review_result.approved else "failed"
    )

    summary = (
        f"Mission {status}. "
        f"Tests: {test_results.get('report', 'N/A')} "
        f"Review: {', '.join(review_result.comments)}"
    )

    return MissionOutput(
        status=status,
        pull_request_url="https://github.com/org/repo/pull/123",  # Mock PR URL
        summary=summary,
    )
