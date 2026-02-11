"""Plan Mission tool — decomposes a high-level goal into a task DAG using LiteLLM."""

from __future__ import annotations

import json
import typing as T

import litellm

from autogen_team.infrastructure.services.mcp_service import MCPService


async def plan_mission(goal: str) -> T.Dict[str, T.Any]:
    """Decompose a high-level goal into a task DAG.

    Args:
        goal: A high-level goal string to decompose.

    Returns:
        A dict representing the task DAG with parallel_tasks array.
    """
    if not goal or not goal.strip():
        return {"goal": "", "parallel_tasks": [], "error": "Empty goal provided"}

    mcp_service = MCPService()
    system_prompt = mcp_service.get_prompt("plan_mission", "system")

    response = await litellm.acompletion(
        model=mcp_service.litellm_model,
        messages=[
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": f"Goal: {goal}"},
        ],
        api_base=mcp_service.litellm_api_base,
        api_key=mcp_service.litellm_api_key,
        response_format={"type": "json_object"},
        temperature=0.2,
    )

    content = response.choices[0].message.content or "{}"

    try:
        dag = json.loads(content)
    except json.JSONDecodeError:
        return {
            "goal": goal,
            "parallel_tasks": [],
            "error": f"Failed to parse LLM response as JSON: {content[:200]}",
        }

    # Validate required fields
    if "parallel_tasks" not in dag:
        dag["parallel_tasks"] = []
    if "goal" not in dag:
        dag["goal"] = goal

    return T.cast(T.Dict[str, T.Any], dag)
