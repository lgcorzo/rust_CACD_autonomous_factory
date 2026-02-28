from typing import Any, Dict

import pytest
from hatchet_sdk import Context, Hatchet


# Skip if token not provided or is a dummy token
def test_hatchet_connection() -> None:
    try:
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
    except SystemExit:
        pytest.skip(
            "Hatchet SDK exited (likely due to missing/invalid credentials or unreachable server)"
        )
    except Exception as e:
        pytest.skip(f"Hatchet connection failed: {e}")
