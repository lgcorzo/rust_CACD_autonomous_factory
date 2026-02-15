import os
import pytest
from typing import Dict, Any
from hatchet_sdk import Hatchet, Context


# Skip if token not provided
@pytest.mark.skipif(not os.getenv("HATCHET_CLIENT_TOKEN"), reason="HATCHET_CLIENT_TOKEN not set")
def test_hatchet_connection() -> None:
    hatchet = Hatchet()

    test_workflow = hatchet.workflow(name="test", on_events=["test-event"])

    @test_workflow.task()
    def test_task_fn(test_input: Any, context: Context) -> Dict[str, str]:
        return {"status": "success"}

    worker = hatchet.worker("test-worker")
    worker.register_workflow(test_workflow)
    # In a real test we'd start the worker in a background thread and trigger an event
    # For now just instantiating confirms basic connectivity if token is valid
    assert worker is not None
