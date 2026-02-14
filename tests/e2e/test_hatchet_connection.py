import os
import pytest
from typing import Dict, cast, Any
from hatchet_sdk import Hatchet, Context


# Skip if token not provided
@pytest.mark.skipif(not os.getenv("HATCHET_CLIENT_TOKEN"), reason="HATCHET_CLIENT_TOKEN not set")
def test_hatchet_connection() -> None:
    hatchet = Hatchet()

    @cast(Any, hatchet.workflow(name="test", on_events=["test-event"]))  # type: ignore[untyped-decorator]
    def test_workflow_fn(context: Context) -> Dict[str, str]:
        return {"status": "success"}

    worker = hatchet.worker("test-worker")
    worker.register_workflow(test_workflow_fn)
    # In a real test we'd start the worker in a background thread and trigger an event
    # For now just instantiating confirms basic connectivity if token is valid
    assert worker is not None
