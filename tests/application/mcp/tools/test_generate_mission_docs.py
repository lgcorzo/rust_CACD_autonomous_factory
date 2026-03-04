"""Tests for generate_mission_docs tool."""

import json
from unittest.mock import AsyncMock, patch

import pytest
from autogen_team.application.mcp.tools.generate_mission_docs import generate_mission_docs


@pytest.mark.asyncio
async def test_generate_mission_docs_success() -> None:
    """Test successful documentation generation."""
    mission_id = "test-mission"
    mission_context = {
        "goal": "Test goal",
        "tasks": [{"id": "t1", "name": "task 1"}],
        "results": [{"status": "success"}],
        "file_changes": ["+ print(1)"],
    }

    mock_response = AsyncMock()
    mock_response.choices = [
        AsyncMock(
            message=AsyncMock(
                content=json.dumps(
                    {
                        "summary": "Test summary",
                        "diagrams": {"sequence": "seq", "class": "cls"},
                    }
                )
            )
        )
    ]

    with (
        patch("litellm.acompletion", return_value=mock_response),
        patch(
            "autogen_team.infrastructure.services.mcp_service.MCPService.get_prompt",
            return_value="prompt",
        ),
    ):
        result = await generate_mission_docs(mission_id, mission_context)

    assert result["summary"] == "Test summary"
    assert result["diagrams"] == {"sequence": "seq", "class": "cls"}


@pytest.mark.asyncio
async def test_generate_mission_docs_empty_context() -> None:
    """Test with empty mission context."""
    result = await generate_mission_docs("id", {})
    assert "error" in result
    assert "Empty mission context" in result["error"]


@pytest.mark.asyncio
async def test_generate_mission_docs_invalid_json() -> None:
    """Test handling of invalid JSON from LLM."""
    mission_id = "test-mission"
    mission_context = {"goal": "test"}

    mock_response = AsyncMock()
    mock_response.choices = [AsyncMock(message=AsyncMock(content="not a json"))]

    with (
        patch("litellm.acompletion", return_value=mock_response),
        patch(
            "autogen_team.infrastructure.services.mcp_service.MCPService.get_prompt",
            return_value="prompt",
        ),
    ):
        result = await generate_mission_docs(mission_id, mission_context)

    assert "error" in result
    assert "Failed to parse LLM response" in result["error"]
