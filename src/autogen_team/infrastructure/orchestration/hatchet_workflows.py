"""Hatchet Workflows for Autogen Team."""

from hatchet_sdk import Context

from autogen_team.application.jobs import inference
from autogen_team.infrastructure.services import HatchetService

hatchet = HatchetService().client


@hatchet.workflow(on_events=["inference:start"])
class InferenceWorkflow:
    """Workflow to execute model inference."""

    @hatchet.step()
    def run_inference(self, context: Context) -> dict:
        """Run the inference job."""
        # Extract parameters from context input
        # Expected input: dict matching InferenceJob fields
        job_params = context.workflow_input()

        # Instantiate and run the job
        with inference.InferenceJob(**job_params) as job:
            results = job.run()

        return {
            "status": "completed",
            "message": "Inference job finished successfully",
            "outputs_shape": str(results.get("outputs", {}).shape if "outputs" in results else "unknown"),
        }
