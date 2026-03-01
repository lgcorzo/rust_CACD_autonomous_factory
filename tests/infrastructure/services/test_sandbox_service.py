"""Tests for SandboxService."""

import pytest
from unittest.mock import MagicMock, patch, AsyncMock
from autogen_team.infrastructure.services.sandbox_service import SandboxService, SandboxExecutionResult

@pytest.fixture
def sandbox_service():
    with patch("autogen_team.infrastructure.services.sandbox_service.E2B_AVAILABLE", True):
        yield SandboxService(use_e2b_fallback=True)

@pytest.mark.asyncio
async def test_create_sandbox_e2b_success(sandbox_service):
    with patch("autogen_team.infrastructure.services.sandbox_service.CodeInterpreter", new_callable=AsyncMock, create=True) as mock_interpreter, \
         patch("autogen_team.infrastructure.services.sandbox_service.E2B_AVAILABLE", True):
        mock_sandbox = AsyncMock()
        mock_interpreter.create.return_value = mock_sandbox
        
        sandbox_id = await sandbox_service.create_sandbox()
        
        assert sandbox_id in sandbox_service.active_sandboxes
        assert sandbox_service.active_sandboxes[sandbox_id] == mock_sandbox
        mock_interpreter.create.assert_called_once()

@pytest.mark.asyncio
async def test_create_sandbox_e2b_failure(sandbox_service):
    with patch("autogen_team.infrastructure.services.sandbox_service.CodeInterpreter", create=True) as mock_interpreter:
        mock_interpreter.create.side_effect = Exception("E2B Error")
        
        with pytest.raises(RuntimeError, match="Sandbox creation failed: E2B Error"):
            await sandbox_service.create_sandbox()

@pytest.mark.asyncio
async def test_create_sandbox_no_fallback(sandbox_service):
    sandbox_service.use_e2b_fallback = False
    with pytest.raises(NotImplementedError, match="Local Firecracker integration not yet implemented"):
        await sandbox_service.create_sandbox()

@pytest.mark.asyncio
async def test_execute_success(sandbox_service):
    mock_sandbox = MagicMock()
    mock_sandbox.notebook.exec_cell.return_value = MagicMock(error=None, results=[MagicMock(text="output")])
    sandbox_service.active_sandboxes["test_id"] = mock_sandbox
    
    result = await sandbox_service.execute("test_id", "echo 'hello'")
    
    assert result.exit_code == 0
    assert result.stdout == "output"
    assert result.stderr == ""

@pytest.mark.asyncio
async def test_execute_error(sandbox_service):
    mock_sandbox = MagicMock()
    mock_sandbox.notebook.exec_cell.return_value = MagicMock(error=MagicMock(value="error"), results=[])
    sandbox_service.active_sandboxes["test_id"] = mock_sandbox
    
    result = await sandbox_service.execute("test_id", "invalid command")
    
    assert result.exit_code == 1
    assert result.stdout == ""
    assert result.stderr == "error"

@pytest.mark.asyncio
async def test_execute_not_found(sandbox_service):
    with pytest.raises(ValueError, match="Sandbox unknown not found"):
        await sandbox_service.execute("unknown", "command")

@pytest.mark.asyncio
async def test_destroy_success(sandbox_service):
    mock_sandbox = AsyncMock()
    sandbox_service.active_sandboxes["test_id"] = mock_sandbox
    
    await sandbox_service.destroy("test_id")
    
    assert "test_id" not in sandbox_service.active_sandboxes
    mock_sandbox.close.assert_called_once()

@pytest.mark.asyncio
async def test_destroy_not_found(sandbox_service):
    # Should not raise exception
    await sandbox_service.destroy("unknown")

@pytest.mark.asyncio
async def test_upload_artifact(sandbox_service):
    with patch("boto3.client") as mock_boto:
        mock_s3 = MagicMock()
        mock_boto.return_value = mock_s3
        
        url = await sandbox_service.upload_artifact("test_id", "/path/to/file", "test-bucket")
        
        assert "test-bucket" in url
        assert "test_id" in url
        mock_s3.upload_file.assert_called_once()

@pytest.mark.asyncio
async def test_run_python_tests(sandbox_service):
    with patch.object(sandbox_service, "execute", new_callable=AsyncMock) as mock_execute:
        await sandbox_service.run_python_tests("test_id", "/workspace")
        mock_execute.assert_called_with("test_id", "cd /workspace && pytest")
