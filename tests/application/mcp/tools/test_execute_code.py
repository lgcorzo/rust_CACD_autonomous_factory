"""Tests for execute_code tool."""

from __future__ import annotations

import json
import typing as T
from unittest.mock import AsyncMock, MagicMock, patch

import pytest

from autogen_team.application.mcp.tools.execute_code import execute_code


@pytest.mark.asyncio
async def test_execute_code_valid_task(sample_task: T.Dict[str, T.Any], tmp_path: str) -> None:
    """Test execute_code generates valid Python files."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = json.dumps(
        {
            "files_changed": [
                {
                    "path": "models/user.py",
                    "action": "create",
                    "content": "class User:\n    def __init__(self, name: str) -> None:\n        self.name = name\n",
                }
            ]
        }
    )

    with patch("autogen_team.application.mcp.tools.execute_code.litellm") as mock_litellm:
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        result = await execute_code(task=sample_task, workspace_path=str(tmp_path))

    assert result["status"] == "success"
    assert len(result["files_changed"]) == 1
    assert result["validation_errors"] == []


@pytest.mark.asyncio
async def test_execute_code_syntax_error(sample_task: T.Dict[str, T.Any], tmp_path: str) -> None:
    """Test execute_code detects Python syntax errors."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = json.dumps(
        {
            "files_changed": [
                {
                    "path": "bad.py",
                    "action": "create",
                    "content": "def foo(\n    # missing closing paren and colon\n",
                }
            ]
        }
    )

    with patch("autogen_team.application.mcp.tools.execute_code.litellm") as mock_litellm:
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        result = await execute_code(task=sample_task, workspace_path=str(tmp_path))

    assert result["status"] == "error"
    assert len(result["validation_errors"]) > 0


@pytest.mark.asyncio
async def test_execute_code_malformed_response(
    sample_task: T.Dict[str, T.Any], tmp_path: str
) -> None:
    """Test execute_code handles malformed LLM response."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = "not json"

    with patch("autogen_team.application.mcp.tools.execute_code.litellm") as mock_litellm:
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        result = await execute_code(task=sample_task, workspace_path=str(tmp_path))

    assert result["status"] == "error"
    assert "Failed to parse" in result["error"]
