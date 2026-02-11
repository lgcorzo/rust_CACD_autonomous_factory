"""Tests for index_code tool."""

from __future__ import annotations

from unittest.mock import AsyncMock, MagicMock, patch

import pytest
from autogen_team.application.mcp.tools.index_code import index_code


@pytest.mark.asyncio
async def test_index_code_success() -> None:
    """Test index_code successfully indexes a file."""
    mock_resp = MagicMock()
    mock_resp.json.return_value = {"results": {"document_id": "doc_abc123"}}
    mock_resp.raise_for_status = MagicMock()

    with patch("autogen_team.application.mcp.tools.index_code.httpx") as mock_httpx:
        mock_client = AsyncMock()
        mock_client.__aenter__ = AsyncMock(return_value=mock_client)
        mock_client.__aexit__ = AsyncMock(return_value=False)
        mock_client.post = AsyncMock(return_value=mock_resp)
        mock_httpx.AsyncClient.return_value = mock_client
        mock_httpx.Timeout = MagicMock()
        mock_httpx.HTTPError = Exception
        mock_httpx.HTTPStatusError = type("HTTPStatusError", (Exception,), {})

        result = await index_code(
            file_path="src/utils.py",
            content="def helper(): pass\n",
            metadata={"language": "python"},
        )

    assert result["status"] == "indexed"
    assert result["document_id"] == "doc_abc123"


@pytest.mark.asyncio
async def test_index_code_empty_content() -> None:
    """Test index_code rejects empty content."""
    result = await index_code(file_path="empty.py", content="")
    assert result["status"] == "error"
    assert "Empty content" in result["error"]


@pytest.mark.asyncio
async def test_index_code_r2r_error() -> None:
    """Test index_code handles R2R API error."""
    with patch("autogen_team.application.mcp.tools.index_code.httpx") as mock_httpx:
        mock_client = AsyncMock()
        mock_client.__aenter__ = AsyncMock(return_value=mock_client)
        mock_client.__aexit__ = AsyncMock(return_value=False)
        mock_client.post = AsyncMock(side_effect=Exception("Server error"))
        mock_httpx.AsyncClient.return_value = mock_client
        mock_httpx.Timeout = MagicMock()
        mock_httpx.HTTPError = Exception
        mock_httpx.HTTPStatusError = type("HTTPStatusError", (Exception,), {})

        result = await index_code(
            file_path="src/utils.py",
            content="def helper(): pass\n",
        )

    assert result["status"] == "error"
    assert "error" in result
