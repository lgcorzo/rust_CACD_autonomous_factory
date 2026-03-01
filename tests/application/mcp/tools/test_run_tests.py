"""Tests for run_tests tool."""

from __future__ import annotations

import typing as T
from unittest.mock import MagicMock, patch, AsyncMock

import pytest
from autogen_team.application.mcp.tools.run_tests import (
    SubprocessSandbox,
    FirecrackerSandbox,
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
    assert (
        "Path traversal detected" in result["details"]
        or "Path traversal attempt" in result["details"]
    )


@pytest.mark.asyncio
async def test_run_tests_delete_action(tmp_path: str) -> None:
    """Test run_tests with delete action."""
    # Create a file in a mock workspace
    workspace = tmp_path / "workspace"
    workspace.mkdir()
    test_file = workspace / "to_delete.py"
    test_file.write_text("print('delete me')")

    changes = {
        "files_changed": [
            {
                "path": "to_delete.py",
                "action": "delete",
            }
        ]
    }

    with (
        patch("autogen_team.application.mcp.tools.run_tests.subprocess") as mock_sub,
        patch("autogen_team.application.mcp.tools.run_tests.os.remove") as mock_remove,
    ):
        mock_sub.run.return_value = MagicMock(returncode=0, stdout="ok", stderr="")
        await run_tests(changes=changes, workspace_path=str(workspace))

    # Verify os.remove was called on the file in the sandbox
    mock_remove.assert_called_once()
    args, _ = mock_remove.call_args
    assert "to_delete.py" in args[0]
    assert "mcp_test_sandbox_" in args[0]


@pytest.mark.asyncio
async def test_firecracker_sandbox_run_tests_success() -> None:
    """Test FirecrackerSandbox.run_tests success path."""
    mock_service = MagicMock()
    mock_service.create_sandbox = AsyncMock(return_value="sb_123")
    mock_service.run_python_tests = AsyncMock(
        return_value=MagicMock(exit_code=0, stdout="pass", stderr="")
    )
    mock_service.destroy = AsyncMock()

    sandbox = FirecrackerSandbox(sandbox_service=mock_service)

    # Mock loop to be not running to test run_until_complete path
    with patch("asyncio.get_event_loop") as mock_loop_get:
        mock_loop = MagicMock()
        mock_loop.is_running.return_value = False

        def side_effect(coro):
            import asyncio

            return asyncio.run(coro)

        mock_loop.run_until_complete.side_effect = side_effect
        mock_loop_get.return_value = mock_loop

        result = sandbox.run_tests(workspace_dir="/tmp/ws")

    assert result["passed"] is True
    assert result["exit_code"] == 0
    mock_service.create_sandbox.assert_called_once()
    mock_service.destroy.assert_called_once_with("sb_123")


@pytest.mark.asyncio
async def test_firecracker_sandbox_run_tests_failure() -> None:
    """Test FirecrackerSandbox.run_tests error handling."""
    mock_service = MagicMock()
    mock_service.create_sandbox = AsyncMock(side_effect=Exception("Creation failed"))

    sandbox = FirecrackerSandbox(sandbox_service=mock_service)

    with patch("asyncio.get_event_loop") as mock_loop_get:
        mock_loop = MagicMock()
        mock_loop.is_running.return_value = False
        mock_loop.run_until_complete.side_effect = lambda coro: asyncio.run(coro)
        mock_loop_get.return_value = mock_loop

        result = sandbox.run_tests(workspace_dir="/tmp/ws")

    assert result["passed"] is False
    assert "Async execution failed" in result["summary"]
