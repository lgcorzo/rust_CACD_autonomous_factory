from typing import Any

from autogen_team.application.jobs import inference
from autogen_team.infrastructure.services import HatchetService
from hatchet_sdk import Context

hatchet = HatchetService().client


# Create Workflow Object
inference_workflow = hatchet.workflow(
    name="inference-workflow",
    on_events=["inference:start"],
)


@inference_workflow.task()
async def run_inference(input: Any, context: Context) -> dict[str, Any]:
    """Run the inference job."""
    # Extract parameters from context input
    # Expected input: dict matching InferenceJob fields
    job_params = context.workflow_input

    # Instantiate and run the job
    with inference.InferenceJob(**job_params) as job:
        results = job.run()

    return {
        "status": "completed",
        "message": "Inference job finished successfully",
        "outputs_shape": str(
            results.get("outputs", {}).shape if "outputs" in results else "unknown"
        ),
    }
