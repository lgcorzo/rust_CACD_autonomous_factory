"""Define a job for triggering a Hatchet inference workflow."""

# %% IMPORTS

import typing as T

import pydantic as pdt

from autogen_team.application.jobs import base
from autogen_team.data_access.adapters import datasets
from autogen_team.infrastructure import services
from autogen_team.registry.adapters import mlflow_adapter as registries

# %% JOBS


class HatchetInferenceJob(base.Job):
    """Trigger a Hatchet inference workflow.

    This job acts as a client-side proxy that starts the asynchronous
    inference process in the Hatchet engine.

    Parameters:
        inputs (datasets.ReaderKind): reader for the inputs data.
        outputs (datasets.WriterKind): writer for the outputs data.
        alias_or_version (str | int): alias or version for the model.
        loader (registries.LoaderKind): registry loader for the model.
        hatchet_service (services.HatchetService): manage the Hatchet system.
    """

    KIND: T.Literal["HatchetInferenceJob"] = "HatchetInferenceJob"

    # Inputs
    inputs: datasets.ReaderKind = pdt.Field(..., discriminator="KIND")
    # Outputs
    outputs: datasets.WriterKind = pdt.Field(..., discriminator="KIND")
    # Model
    alias_or_version: str | int = "Champion"
    # Loader
    loader: registries.LoaderKind = pdt.Field(registries.CustomLoader(), discriminator="KIND")
    # Hatchet
    hatchet_service: services.HatchetService = services.HatchetService()

    def run(self) -> base.Locals:
        # services
        logger = self.logger_service.logger()
        logger.info("Executing Hatchet Inference Job (Proxy)")

        # Prepare payload for Hatchet
        # We need to serialize the job parameters so the worker can reconstruct the job.
        # Since ReaderKind and WriterKind are pydantic models, we can use model_dump.
        workflow_input = {
            "inputs": self.inputs.model_dump(),
            "outputs": self.outputs.model_dump(),
            "alias_or_version": self.alias_or_version,
            "loader": self.loader.model_dump(),
        }

        logger.debug("Triggering workflow 'InferenceWorkflow' with input: {}", workflow_input)

        # Trigger Hatchet Workflow
        # The HatchetService provides the client.
        try:
            self.hatchet_service.client.admin.run_workflow("InferenceWorkflow", workflow_input)
            logger.info("Successfully triggered Hatchet Inference Workflow")

            self.alerts_service.notify(
                title="Hatchet Inference Triggered",
                message=f"Workflow 'InferenceWorkflow' started for model {self.alias_or_version}",
            )
        except Exception as e:
            logger.error("Failed to trigger Hatchet workflow: {}", str(e))
            self.alerts_service.notify(
                title="Hatchet Inference Failed",
                message=f"Error triggering workflow: {str(e)}",
            )
            raise

        return locals()
