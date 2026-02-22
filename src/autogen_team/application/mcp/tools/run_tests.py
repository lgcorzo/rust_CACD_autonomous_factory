"""Run Tests tool — runs pytest in an isolated sandbox."""

from __future__ import annotations

import abc
import os
import shutil
import subprocess  # nosec B404
import sys
import tempfile
import typing as T

from autogen_team.core.security import safe_join


class SandboxBackend(abc.ABC):
    """Abstract sandbox backend for running tests.

    Provides an interface for future Firecracker MicroVM integration.
    """

    @abc.abstractmethod
    def run_tests(
        self,
        workspace_dir: str,
        timeout: int = 300,
    ) -> T.Dict[str, T.Any]:
        """Run tests in the sandbox.

        Args:
            workspace_dir: Path to the workspace with changes applied.
            timeout: Maximum execution time in seconds.

        Returns:
            Dict with passed, summary, and details fields.
        """


class SubprocessSandbox(SandboxBackend):
    """Subprocess-based sandbox for running pytest."""

    def run_tests(
        self,
        workspace_dir: str,
        timeout: int = 300,
    ) -> T.Dict[str, T.Any]:
        """Run pytest via subprocess in the given workspace.

        Args:
            workspace_dir: Path to the workspace with changes applied.
            timeout: Maximum execution time in seconds.

        Returns:
            Dict with passed, summary, and details fields.
        """
        try:
            result = subprocess.run(  # nosec B603
                [sys.executable, "-m", "pytest", "--tb=short", "-q", "--no-header"],
                cwd=workspace_dir,
                capture_output=True,
                text=True,
                timeout=timeout,
            )

            passed = result.returncode == 0
            output = result.stdout + result.stderr

            return {
                "passed": passed,
                "summary": f"Tests {'passed' if passed else 'failed'} (exit code {result.returncode})",
                "details": output[-2000:] if len(output) > 2000 else output,
                "exit_code": result.returncode,
            }

        except subprocess.TimeoutExpired:
            return {
                "passed": False,
                "summary": f"Tests timed out after {timeout}s",
                "details": "Process killed due to timeout.",
                "exit_code": -1,
            }
        except Exception as e:
            return {
                "passed": False,
                "summary": f"Test execution error: {type(e).__name__}",
                "details": str(e),
                "exit_code": -1,
            }


# Default sandbox implementation
_default_sandbox: SandboxBackend = SubprocessSandbox()


async def run_tests(
    changes: T.Dict[str, T.Any],
    workspace_path: str = "",
    timeout: int = 300,
    sandbox: SandboxBackend | None = None,
) -> T.Dict[str, T.Any]:
    """Run pytest against code changes in an isolated sandbox.

    Args:
        changes: Dict with files_changed list (path, action, content).
        workspace_path: Original workspace path to copy from.
        timeout: Max execution time in seconds.
        sandbox: Optional sandbox backend (defaults to SubprocessSandbox).

    Returns:
        Dict with passed bool, summary string, and details.
    """
    backend = sandbox or _default_sandbox
    files_changed = changes.get("files_changed", [])

    sandbox_dir = tempfile.mkdtemp(prefix="mcp_test_sandbox_")

    try:
        # Copy original workspace if provided
        if workspace_path and os.path.isdir(workspace_path):
            shutil.copytree(workspace_path, sandbox_dir, dirs_exist_ok=True)

        # Apply changes
        for file_change in files_changed:
            file_path = file_change.get("path", "")
            action = file_change.get("action", "create")
            content = file_change.get("content", "")

            try:
                full_path = safe_join(sandbox_dir, file_path)
            except ValueError as e:
                return {
                    "passed": False,
                    "summary": f"Security Error: {e}",
                    "details": str(e),
                    "exit_code": -1,
                }

            if action == "delete":
                if os.path.exists(full_path):
                    os.remove(full_path)
                continue

            os.makedirs(os.path.dirname(full_path), exist_ok=True)
            with open(full_path, "w") as f:
                f.write(content)

        # Run tests
        result = backend.run_tests(workspace_dir=sandbox_dir, timeout=timeout)

    finally:
        shutil.rmtree(sandbox_dir, ignore_errors=True)

    return T.cast(T.Dict[str, T.Any], result)
