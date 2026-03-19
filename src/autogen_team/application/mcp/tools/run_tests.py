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


class FirecrackerSandbox(SandboxBackend):
    """Firecracker-based sandbox using SandboxService."""

    def __init__(self, sandbox_service: T.Any | None = None):
        from autogen_team.infrastructure.services.sandbox_service import SandboxService

        self.service = sandbox_service or SandboxService()

    def run_tests(
        self,
        workspace_dir: str,
        timeout: int = 300,
    ) -> T.Dict[str, T.Any]:
        """Note: This is a synchronous wrapper for the async service.
        In a real scenario, the tool should be async.
        """
        import asyncio

        async def _run() -> T.Dict[str, T.Any]:
            sandbox_id = await self.service.create_sandbox()
            try:
                # Note: In a production environment, we would also sync files to the sandbox
                # For now, we implement the interface for the backend.
                result = await self.service.run_python_tests(sandbox_id, workspace_dir)
                return {
                    "passed": result.exit_code == 0,
                    "summary": f"Sandbox tests {'passed' if result.exit_code == 0 else 'failed'}",
                    "details": result.stdout + result.stderr,
                    "exit_code": result.exit_code,
                }
            except Exception as e:
                return {
                    "passed": False,
                    "summary": f"Sandbox error: {type(e).__name__}",
                    "details": str(e),
                    "exit_code": -1,
                }
            finally:
                await self.service.destroy(sandbox_id)

        # Run the async logic in the current thread (assuming it's not already in an event loop)
        # or use the existing loop if available.
        try:
            loop = asyncio.get_event_loop()
            if loop.is_running():
                # This is tricky in sync tools, better if the whole tool becomes async
                # For this implementation, we assume the tool will be called correctly.
                return T.cast(
                    "T.Dict[str, T.Any]",
                    asyncio.run_coroutine_threadsafe(_run(), loop).result(timeout=timeout),
                )
            else:
                return loop.run_until_complete(_run())
        except Exception as e:
            return {
                "passed": False,
                "summary": "Async execution failed in sandbox",
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
        try:
            for file_change in files_changed:
                file_path = file_change.get("path", "")
                action = file_change.get("action", "create")
                content = file_change.get("content", "")

                try:
                    full_path = safe_join(sandbox_dir, file_path)
                except ValueError as e:
                    return {
                        "passed": False,
                        "summary": f"Security Error: {file_path}: {e}",
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
        except ValueError as e:
            return {
                "passed": False,
                "summary": f"Security Error: {str(e)}",
                "details": str(e),
                "exit_code": -1,
            }

    finally:
        shutil.rmtree(sandbox_dir, ignore_errors=True)

    return T.cast(T.Dict[str, T.Any], result)
