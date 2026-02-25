"""Tests for run_tests tool."""

from __future__ import annotations

import typing as T
from unittest.mock import MagicMock, patch

import pytest
from autogen_team.application.mcp.tools.run_tests import (
    SubprocessSandbox,
    run_tests,
)


@pytest.mark.asyncio
async def test_run_tests_passing(sample_changes: T.Dict[str, T.Any], tmp_path: str) -> None:
    """Test run_tests with passing tests."""
    mock_result = MagicMock()
    mock_result.returncode = 0
    mock_result.stdout = "1 passed"
    mock_result.stderr = ""

    with patch("autogen_team.application.mcp.tools.run_tests.subprocess") as mock_sub:
        mock_sub.run.return_value = mock_result
        result = await run_tests(changes=sample_changes, workspace_path=str(tmp_path))

    assert result["passed"] is True
    assert "passed" in result["summary"]


@pytest.mark.asyncio
async def test_run_tests_failing(sample_changes: T.Dict[str, T.Any], tmp_path: str) -> None:
    """Test run_tests with failing tests."""
    mock_result = MagicMock()
    mock_result.returncode = 1
    mock_result.stdout = "1 failed"
    mock_result.stderr = ""

    with patch("autogen_team.application.mcp.tools.run_tests.subprocess") as mock_sub:
        mock_sub.run.return_value = mock_result
        result = await run_tests(changes=sample_changes, workspace_path=str(tmp_path))

    assert result["passed"] is False
    assert "failed" in result["summary"]


@pytest.mark.asyncio
async def test_run_tests_timeout(sample_changes: T.Dict[str, T.Any], tmp_path: str) -> None:
    """Test run_tests handles subprocess timeout."""
    import subprocess

    with patch("autogen_team.application.mcp.tools.run_tests.subprocess") as mock_sub:
        mock_sub.run.side_effect = subprocess.TimeoutExpired(cmd="pytest", timeout=5)
        mock_sub.TimeoutExpired = subprocess.TimeoutExpired
        result = await run_tests(changes=sample_changes, workspace_path=str(tmp_path), timeout=5)

    assert result["passed"] is False
    assert "timed out" in result["summary"]


def test_subprocess_sandbox_direct() -> None:
    """Test SubprocessSandbox.run_tests returns expected structure."""
    sandbox = SubprocessSandbox()
    # Run against a non-existent dir so pytest fails quickly
    result = sandbox.run_tests(workspace_dir="/tmp/nonexistent_test_dir_xyz")
    assert "passed" in result
    assert "summary" in result
    assert isinstance(result["passed"], bool)


@pytest.mark.asyncio
async def test_run_tests_path_traversal(sample_changes: T.Dict[str, T.Any], tmp_path: str) -> None:
    """Test run_tests prevents path traversal."""
    changes = {
        "files_changed": [
            {
                "path": "../escape.py",
                "action": "create",
                "content": "print('hacked')",
            }
        ]
    }

    result = await run_tests(changes=changes, workspace_path=str(tmp_path))

    assert result["passed"] is False
    assert "Security Error" in result["summary"]
    assert "Path traversal detected" in result["details"] or "Path traversal attempt" in result["details"]
