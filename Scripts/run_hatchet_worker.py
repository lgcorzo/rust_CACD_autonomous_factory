"""Script to run the Hatchet worker and register workflows."""

from autogen_team.application.workflows.autonomous_mission import autonomous_mission_workflow
from autogen_team.infrastructure.services.hatchet_service import HatchetService


def main() -> None:
    print("🚀 Starting Hatchet Worker...")

    # Initialize Service
    service = HatchetService()
    hatchet = service.client

    # Register workflow
    # Note: In Hatchet Python SDK, typically we use a Worker object to register workflows
    worker = hatchet.worker("autogen-worker")
    worker.register_workflow(autonomous_mission_workflow)

    print("✅ Worker initialized. Listening for tasks...")
    worker.start()


if __name__ == "__main__":
    main()
