"""Generate Mission Documentation tool — creates Mermaid diagrams from mission results."""

from __future__ import annotations

import json
import typing as T

import litellm

from autogen_team.infrastructure.services.mcp_service import MCPService


async def generate_mission_docs(
    mission_id: str, mission_context: T.Dict[str, T.Any]
) -> T.Dict[str, T.Any]:
    """Generate Mermaid diagrams and documentation for a mission.

    Args:
        mission_id: Unique identifier for the mission.
        mission_context: Context including goal, tasks, results, and file changes.

    Returns:
        A dict containing generated Mermaid diagrams and documentation.
    """
    if not mission_context:
        return {"mission_id": mission_id, "error": "Empty mission context provided"}

    mcp_service = MCPService()
    system_prompt = mcp_service.get_prompt("generate_mission_docs", "system")
    instructions = mcp_service.get_prompt("generate_mission_docs", "instructions")

    # Format instructions with mission context
    formatted_instructions = instructions.format(
        mission_id=mission_id,
        goal=mission_context.get("goal", "N/A"),
        tasks=json.dumps(mission_context.get("tasks", []), indent=2),
        results=json.dumps(mission_context.get("results", []), indent=2),
        file_changes=json.dumps(mission_context.get("file_changes", []), indent=2),
    )

    response = await litellm.acompletion(
        model=mcp_service.litellm_model,
        messages=[
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": formatted_instructions},
        ],
        api_base=mcp_service.litellm_api_base,
        api_key=mcp_service.litellm_api_key,
        response_format={"type": "json_object"},
        temperature=0.2,
    )

    content = response.choices[0].message.content or "{}"

    try:
        docs = json.loads(content)
    except json.JSONDecodeError:
        return {
            "mission_id": mission_id,
            "error": f"Failed to parse LLM response as JSON: {content[:200]}",
        }

    # Ensure required fields exist
    if "diagrams" not in docs:
        docs["diagrams"] = {}
    if "summary" not in docs:
        docs["summary"] = "No summary generated."

    return T.cast(T.Dict[str, T.Any], docs)
