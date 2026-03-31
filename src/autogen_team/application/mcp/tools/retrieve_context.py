"""Retrieve Context tool — queries R2R RAG for relevant codebase patterns."""

from __future__ import annotations

import typing as T

from loguru import logger

import httpx
from loguru import logger

from autogen_team.infrastructure.io.osvariables import Env


async def retrieve_context(
    query: str,
    collection_name: str = "default",
) -> T.Dict[str, T.Any]:
    """Query R2R RAG system for relevant codebase patterns via semantic search.

    Args:
        query: Search query string.
        collection_name: Name of the R2R collection to search.

    Returns:
        Dict with matching documents and graph context.
    """
    if not query or not query.strip():
        return {"documents": [], "graph_context": {}, "error": "Empty query"}

    env = Env()

    try:
        async with httpx.AsyncClient(
            base_url=env.r2r_base_url, timeout=httpx.Timeout(30.0)
        ) as client:
            response = await client.post(
                "/v3/retrieval/search",
                json={
                    "query": query,
                    "search_settings": {
                        "limit": 10,
                        "filters": {"collection_ids": [collection_name]},
                        "graph_settings": {
                            "enabled": True,
                        },
                    },
                },
            )
            response.raise_for_status()
            data = response.json()

    except httpx.HTTPStatusError as e:
        logger.error(f"R2R API HTTP status error: {e.response.status_code} - {e.response.text}")
        return {
            "documents": [],
            "graph_context": {},
            "error": "R2R API error: Internal server error",
        }
    except httpx.HTTPError as e:
        logger.error(f"R2R connection error: {type(e).__name__}: {e!s}")
        return {
            "documents": [],
            "graph_context": {},
            "error": "R2R connection error: Internal server error",
        }

    results = data.get("results", {})
    chunk_results = results.get("chunk_search_results", [])
    graph_results = results.get("graph_search_results", [])

    documents = [
        {
            "id": doc.get("id", ""),
            "text": doc.get("text", ""),
            "score": doc.get("score", 0.0),
            "metadata": doc.get("metadata", {}),
        }
        for doc in chunk_results
    ]

    return {
        "documents": documents,
        "graph_context": {
            "entities": graph_results,
        },
    }
