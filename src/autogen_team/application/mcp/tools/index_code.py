"""Index Code tool — indexes code files into R2R knowledge graph."""

from __future__ import annotations

import typing as T

import httpx

from autogen_team.infrastructure.io.osvariables import Env


async def index_code(
    file_path: str,
    content: str,
    metadata: T.Dict[str, T.Any] | None = None,
) -> T.Dict[str, T.Any]:
    """Index a code file into R2R knowledge graph for future retrieval.

    Args:
        file_path: Path of the file being indexed.
        content: Full content of the file.
        metadata: Optional metadata dict (language, author, etc).

    Returns:
        Dict with document_id and status.
    """
    if not content or not content.strip():
        return {"document_id": "", "status": "error", "error": "Empty content"}

    env = Env()
    file_metadata = metadata or {}
    file_metadata["file_path"] = file_path

    try:
        async with httpx.AsyncClient(
            base_url=env.r2r_base_url, timeout=httpx.Timeout(30.0)
        ) as client:
            response = await client.post(
                "/v3/documents",
                json={
                    "raw_text": content,
                    "metadata": file_metadata,
                },
            )
            response.raise_for_status()
            data = response.json()

    except httpx.HTTPStatusError as e:
        return {
            "document_id": "",
            "status": "error",
            "error": f"R2R API error: {e.response.status_code} - {e.response.text[:200]}",
        }
    except httpx.HTTPError as e:
        return {
            "document_id": "",
            "status": "error",
            "error": f"R2R connection error: {type(e).__name__}: {e!s}",
        }

    results = data.get("results", {})
    document_id = results.get("document_id", "")

    return {
        "document_id": document_id,
        "status": "indexed",
    }
