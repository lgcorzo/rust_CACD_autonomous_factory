"""Test fixtures for MCP tests."""

import typing as T

import pytest


@pytest.fixture
def sample_goal() -> str:
    """Return a sample goal for plan_mission tests."""
    return "Build a REST API for user management with authentication"


@pytest.fixture
def sample_task() -> T.Dict[str, T.Any]:
    """Return a sample task dict for execute_code tests."""
    return {
        "id": "task_1",
        "name": "Create user model",
        "description": "Create a User model with id, name, email fields.",
    }


@pytest.fixture
def sample_diff() -> str:
    """Return a sample code diff for security_review tests."""
    return """
+import os
+
+def get_user(user_id: int) -> dict:
+    return {"id": user_id, "name": "test"}
"""


@pytest.fixture
def sample_changes() -> T.Dict[str, T.Any]:
    """Return sample file changes for run_tests tests."""
    return {
        "files_changed": [
            {
                "path": "test_sample.py",
                "action": "create",
                "content": "def test_pass(): assert True\n",
            }
        ]
    }


@pytest.fixture
def insecure_diff() -> str:
    """Return a diff with security issues for testing."""
    return """
+import os
+import pickle
+
+def run_cmd(user_input: str) -> None:
+    os.system(f"echo {user_input}")
+
+def load_data(data: bytes) -> object:
+    return pickle.loads(data)
"""
