"""Script to trigger the Autonomous Mission Workflow."""

import asyncio
from autogen_team.infrastructure.services.hatchet_service import HatchetService


async def main() -> None:
    print("🚀 Triggering Autonomous Mission Workflow...")

    service = HatchetService()
    hatchet = service.client

    # Trigger the workflow
    # Note: 'admin' property is not exposed on Hatchet wrapper in this version, so accessing via _client
    # Using aio_run_workflow for async execution
    workflow_run_ref = await hatchet._client.admin.aio_run_workflow(
        "AutonomousMissionWorkflow",
        {"goal": "Create a simple README.md file", "repository_path": "."},
    )
    workflow_run_id = workflow_run_ref.workflow_run_id

    print(f"✅ Workflow triggered! Run ID: {workflow_run_id}")
    print("Check Hatchet Dashboard for progress.")


if __name__ == "__main__":
    asyncio.run(main())
