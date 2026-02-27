"""Tests for security_review tool."""

from __future__ import annotations

import json
from unittest.mock import AsyncMock, MagicMock, patch

import pytest
from autogen_team.application.mcp.tools.security_review import (
    _scan_owasp_patterns,
    security_review,
)


def test_owasp_scan_clean_code() -> None:
    """Test OWASP scanner with clean code."""
    diff = "+def add(a: int, b: int) -> int:\n+    return a + b\n"
    findings = _scan_owasp_patterns(diff)
    assert len(findings) == 0


def test_owasp_scan_command_injection() -> None:
    """Test OWASP scanner detects command injection."""
    diff = '+os.system(f"rm -rf {user_input}")\n'
    findings = _scan_owasp_patterns(diff)
    assert len(findings) > 0
    assert any("Injection" in f["rule"] for f in findings)


def test_owasp_scan_unsafe_deserialization() -> None:
    """Test OWASP scanner detects pickle.loads."""
    diff = "+data = pickle.loads(raw_bytes)\n"
    findings = _scan_owasp_patterns(diff)
    assert len(findings) > 0
    assert any("Deserialization" in f["rule"] for f in findings)


def test_owasp_scan_weak_hash() -> None:
    """Test OWASP scanner detects weak hash usage."""
    diff = "+hash_val = hashlib.md5(data)\n"
    findings = _scan_owasp_patterns(diff)
    assert len(findings) > 0
    assert any("Cryptographic Failures" in f["rule"] for f in findings)


@pytest.mark.asyncio
async def test_security_review_clean_diff(sample_diff: str) -> None:
    """Test security_review approves clean code."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = json.dumps(
        {
            "additional_findings": [],
            "verdict": "approved",
            "summary": "No issues found.",
        }
    )

    with (
        patch("autogen_team.application.mcp.tools.security_review.litellm") as mock_litellm,
        patch("autogen_team.application.mcp.tools.security_review.httpx") as mock_httpx,
    ):
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        # Mock R2R client
        mock_client = AsyncMock()
        mock_resp = MagicMock()
        mock_resp.json.return_value = {"results": {"chunk_search_results": []}}
        mock_resp.raise_for_status = MagicMock()
        mock_client.__aenter__ = AsyncMock(return_value=mock_client)
        mock_client.__aexit__ = AsyncMock(return_value=False)
        mock_client.post = AsyncMock(return_value=mock_resp)
        mock_httpx.AsyncClient.return_value = mock_client
        mock_httpx.Timeout = MagicMock()
        mock_httpx.HTTPError = Exception

        result = await security_review(sample_diff)

    assert result["status"] == "approved"


@pytest.mark.asyncio
async def test_security_review_insecure_diff(insecure_diff: str) -> None:
    """Test security_review rejects insecure code."""
    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = json.dumps(
        {
            "additional_findings": [],
            "verdict": "rejected",
            "summary": "Security issues found.",
        }
    )

    with (
        patch("autogen_team.application.mcp.tools.security_review.litellm") as mock_litellm,
        patch("autogen_team.application.mcp.tools.security_review.httpx") as mock_httpx,
    ):
        mock_litellm.acompletion = AsyncMock(return_value=mock_response)
        mock_client = AsyncMock()
        mock_resp = MagicMock()
        mock_resp.json.return_value = {"results": {"chunk_search_results": []}}
        mock_resp.raise_for_status = MagicMock()
        mock_client.__aenter__ = AsyncMock(return_value=mock_client)
        mock_client.__aexit__ = AsyncMock(return_value=False)
        mock_client.post = AsyncMock(return_value=mock_resp)
        mock_httpx.AsyncClient.return_value = mock_client
        mock_httpx.Timeout = MagicMock()
        mock_httpx.HTTPError = Exception

        result = await security_review(insecure_diff)

    # High-severity OWASP findings always reject
    assert result["status"] == "rejected"
    assert len(result["findings"]) > 0


@pytest.mark.asyncio
async def test_security_review_empty_diff() -> None:
    """Test security_review with empty diff."""
    result = await security_review("")
    assert result["status"] == "approved"
    assert result["note"] == "Empty diff"
