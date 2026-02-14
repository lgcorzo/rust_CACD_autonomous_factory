"""Script to run the Hatchet worker and register workflows."""

from hatchet_sdk import Hatchet
from autogen_team.infrastructure.services.hatchet_service import HatchetService
from autogen_team.application.workflows.autonomous_mission import autonomous_mission_workflow

def main():
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
