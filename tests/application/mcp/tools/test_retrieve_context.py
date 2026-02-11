"""Tests for retrieve_context tool."""

from __future__ import annotations

from unittest.mock import AsyncMock, MagicMock, patch

import pytest
from autogen_team.application.mcp.tools.retrieve_context import retrieve_context


@pytest.mark.asyncio
async def test_retrieve_context_valid_query() -> None:
    """Test retrieve_context returns documents for a valid query."""
    mock_resp = MagicMock()
    mock_resp.json.return_value = {
        "results": {
            "chunk_search_results": [
                {
                    "id": "doc_1",
                    "text": "Example code pattern",
                    "score": 0.95,
                    "metadata": {"file": "utils.py"},
                },
            ],
            "graph_search_results": [
                {"entity": "UserService", "type": "class"},
            ],
        }
    }
    mock_resp.raise_for_status = MagicMock()

    with patch("autogen_team.application.mcp.tools.retrieve_context.httpx") as mock_httpx:
        mock_client = AsyncMock()
        mock_client.__aenter__ = AsyncMock(return_value=mock_client)
        mock_client.__aexit__ = AsyncMock(return_value=False)
        mock_client.post = AsyncMock(return_value=mock_resp)
        mock_httpx.AsyncClient.return_value = mock_client
        mock_httpx.Timeout = MagicMock()
        mock_httpx.HTTPError = Exception
        mock_httpx.HTTPStatusError = type("HTTPStatusError", (Exception,), {})

        result = await retrieve_context(query="user service pattern")

    assert len(result["documents"]) == 1
    assert result["documents"][0]["id"] == "doc_1"
    assert len(result["graph_context"]["entities"]) == 1


@pytest.mark.asyncio
async def test_retrieve_context_empty_query() -> None:
    """Test retrieve_context with empty query returns error."""
    result = await retrieve_context(query="")
    assert result["error"] == "Empty query"
    assert result["documents"] == []


@pytest.mark.asyncio
async def test_retrieve_context_r2r_error() -> None:
    """Test retrieve_context handles R2R connection error."""
    with patch("autogen_team.application.mcp.tools.retrieve_context.httpx") as mock_httpx:
        mock_client = AsyncMock()
        mock_client.__aenter__ = AsyncMock(return_value=mock_client)
        mock_client.__aexit__ = AsyncMock(return_value=False)
        mock_client.post = AsyncMock(side_effect=Exception("Connection refused"))
        mock_httpx.AsyncClient.return_value = mock_client
        mock_httpx.Timeout = MagicMock()
        mock_httpx.HTTPError = Exception
        mock_httpx.HTTPStatusError = type("HTTPStatusError", (Exception,), {})

        result = await retrieve_context(query="test query")

    assert "error" in result
    assert result["documents"] == []
