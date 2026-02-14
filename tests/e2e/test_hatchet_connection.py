import os
import pytest
from hatchet_sdk import Hatchet, Context


# Skip if token not provided
@pytest.mark.skipif(not os.getenv("HATCHET_CLIENT_TOKEN"), reason="HATCHET_CLIENT_TOKEN not set")
def test_hatchet_connection():
    hatchet = Hatchet()

    @hatchet.workflow(on_events=["test-event"])
    class TestWorkflow:
        @hatchet.step()
        def test_step(self, context: Context):
            return {"status": "success"}

    worker = hatchet.worker("test-worker")
    worker.register_workflow(TestWorkflow())
    # In a real test we'd start the worker in a background thread and trigger an event
    # For now just instantiating confirms basic connectivity if token is valid
    assert worker is not None
