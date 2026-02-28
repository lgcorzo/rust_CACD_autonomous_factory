"""Sandbox Service — manages the lifecycle of hardware-isolated MicroVMs."""

from __future__ import annotations

import os
import typing as T
import uuid

import boto3
from loguru import logger

try:
    from e2b_code_interpreter import CodeInterpreter

    E2B_AVAILABLE = True
except ImportError:
    E2B_AVAILABLE = False


class SandboxExecutionResult:
    """Result of a command execution inside the sandbox."""

    def __init__(
        self,
        exit_code: int,
        stdout: str,
        stderr: str,
        artifacts: T.List[str] | None = None,
    ):
        self.exit_code = exit_code
        self.stdout = stdout
        self.stderr = stderr
        self.artifacts = artifacts or []


class SandboxService:
    """Manages ephemeral MicroVM sandboxes for secure code execution."""

    def __init__(self, use_e2b_fallback: bool = True):
        self.use_e2b_fallback = use_e2b_fallback
        self.active_sandboxes: T.Dict[str, T.Any] = {}
        self._execution_timeout = int(os.getenv("SANDBOX_TIMEOUT_SECONDS", "300"))

    async def create_sandbox(
        self,
        metadata: T.Dict[str, T.Any] | None = None,
    ) -> str:
        """Create a new sandbox instance.

        Args:
            metadata: Optional metadata for the sandbox.

        Returns:
            sandbox_id: Unique identifier for the sandbox.
        """
        sandbox_id = str(uuid.uuid4())
        logger.info(f"Creating sandbox {sandbox_id}")

        if E2B_AVAILABLE and self.use_e2b_fallback:
            try:
                # E2B handles the underlying MicroVM (Firecracker-based)
                sandbox = await CodeInterpreter.create()
                self.active_sandboxes[sandbox_id] = sandbox
                logger.info(f"Successfully created E2B sandbox for {sandbox_id}")
                return sandbox_id
            except Exception as e:
                logger.error(f"Failed to create E2B sandbox: {e}")
                raise RuntimeError(f"Sandbox creation failed: {e}")

        # Local Firecracker implementation would go here
        # For DA-14, we start with E2B fallback and infrastructure for local connection
        raise NotImplementedError(
            "Local Firecracker integration not yet implemented in this cluster."
        )

    async def execute(
        self,
        sandbox_id: str,
        command: str,
    ) -> SandboxExecutionResult:
        """Execute a command inside the specified sandbox.

        Args:
            sandbox_id: The ID of the sandbox.
            command: The command to execute.

        Returns:
            SandboxExecutionResult object.
        """
        sandbox = self.active_sandboxes.get(sandbox_id)
        if not sandbox:
            raise ValueError(f"Sandbox {sandbox_id} not found or already destroyed.")

        logger.info(f"Executing command in sandbox {sandbox_id}: {command}")

        if E2B_AVAILABLE and self.use_e2b_fallback:
            # E2B execution
            execution = sandbox.notebook.exec_cell(command)
            return SandboxExecutionResult(
                exit_code=0 if not execution.error else 1,
                stdout=execution.results[0].text if execution.results else "",
                stderr=execution.error.value if execution.error else "",
            )

        raise NotImplementedError("Execution logic for requested backend not implemented.")

    async def run_python_tests(
        self,
        sandbox_id: str,
        workspace_dir: str,
    ) -> SandboxExecutionResult:
        """Specific helper to run pytest inside the sandbox.

        Args:
            sandbox_id: The ID of the sandbox.
            workspace_dir: The directory inside the sandbox where code is located.

        Returns:
            SandboxExecutionResult object.
        """
        # In E2B, we might need to upload files first if they aren't there
        # but the run_tests tool usually handles the workspace structure.
        return await self.execute(sandbox_id, f"cd {workspace_dir} && pytest")

    async def destroy(self, sandbox_id: str) -> None:
        """Tear down a sandbox instance.

        Args:
            sandbox_id: The ID of the sandbox to destroy.
        """
        sandbox = self.active_sandboxes.pop(sandbox_id, None)
        if sandbox:
            logger.info(f"Destroying sandbox {sandbox_id}")
            if E2B_AVAILABLE and self.use_e2b_fallback:
                await sandbox.close()
            logger.info(f"Sandbox {sandbox_id} destroyed.")
        else:
            logger.warning(f"Attempted to destroy non-existent sandbox {sandbox_id}")

    async def upload_artifact(
        self,
        sandbox_id: str,
        file_path: str,
        bucket_name: str = "agent-workspace",
    ) -> str:
        """Upload a file from the local environment (captured from sandbox) to MinIO.

        Args:
            sandbox_id: The ID of the sandbox.
            file_path: Local path to the file.
            bucket_name: Target bucket name.

        Returns:
            The S3 URL of the uploaded artifact.
        """
        s3_endpoint = os.getenv(
            "MLFLOW_S3_ENDPOINT_URL", "http://mlflow-minio-hl.storage.svc.cluster.local:9000"
        )
        access_key = os.getenv("AWS_ACCESS_KEY_ID")
        secret_key = os.getenv("AWS_SECRET_ACCESS_KEY")

        s3 = boto3.client(
            "s3",
            endpoint_url=s3_endpoint,
            aws_access_key_id=access_key,
            aws_secret_access_key=secret_key,
        )

        object_name = f"sandboxes/{sandbox_id}/{os.path.basename(file_path)}"
        s3.upload_file(file_path, bucket_name, object_name)

        url = f"{s3_endpoint}/{bucket_name}/{object_name}"
        logger.info(f"Uploaded artifact for sandbox {sandbox_id} to {url}")
        return url
