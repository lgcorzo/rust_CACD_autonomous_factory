"""Execute Code tool — generates code changes and validates in sandbox."""

from __future__ import annotations

import json
import os
import py_compile
import shutil
import tempfile
import typing as T

import litellm

from autogen_team.infrastructure.services.mcp_service import MCPService


async def execute_code(
    task: T.Dict[str, T.Any],
    workspace_path: str,
) -> T.Dict[str, T.Any]:
    """Generate code changes for a task and validate in sandbox.

    Args:
        task: A task dict (from DAG) with id, name, description.
        workspace_path: Path to the workspace root.

    Returns:
        A dict with files_changed list and status.
    """

    task_description = task.get("description", task.get("name", ""))
    task_name = task.get("name", "unknown_task")

    # Gather workspace context (list of Python files)
    py_files: T.List[str] = []
    for root, _dirs, files in os.walk(workspace_path):
        for filename in files:
            if filename.endswith(".py"):
                rel = os.path.relpath(os.path.join(root, filename), workspace_path)
                py_files.append(rel)

    context = "Workspace files:\n" + "\n".join(py_files[:50])

    mcp_service = MCPService()
    system_prompt = mcp_service.get_prompt("execute_code", "system")

    response = await litellm.acompletion(
        model=mcp_service.litellm_model,
        messages=[
            {"role": "system", "content": system_prompt},
            {
                "role": "user",
                "content": (
                    f"Task: {task_name}\n"
                    f"Description: {task_description}\n\n"
                    f"Context:\n{context}"
                ),
            },
        ],
        api_base=mcp_service.litellm_api_base,
        api_key=mcp_service.litellm_api_key,
        response_format={"type": "json_object"},
        temperature=0.1,
    )

    content = response.choices[0].message.content or "{}"

    try:
        changes = json.loads(content)
    except json.JSONDecodeError:
        return {
            "files_changed": [],
            "status": "error",
            "error": f"Failed to parse LLM response: {content[:200]}",
        }

    files_changed = changes.get("files_changed", [])

    # Validate in sandbox
    sandbox_dir = tempfile.mkdtemp(prefix="mcp_sandbox_")
    validation_errors: T.List[str] = []

    try:
        # Copy workspace structure for validation
        for file_change in files_changed:
            file_path = file_change.get("path", "")
            action = file_change.get("action", "create")
            file_content = file_change.get("content", "")

            if action == "delete":
                continue

            full_path = os.path.abspath(os.path.join(sandbox_dir, file_path))
            sandbox_abs = os.path.abspath(sandbox_dir)
            if not full_path.startswith(sandbox_abs) or (
                len(full_path) > len(sandbox_abs) and full_path[len(sandbox_abs)] != os.sep
            ):
                validation_errors.append(f"Security Error: Invalid path traversal detected: {file_path}")
                continue

            os.makedirs(os.path.dirname(full_path), exist_ok=True)

            with open(full_path, "w") as f:
                f.write(file_content)

            # Validate Python syntax
            if file_path.endswith(".py"):
                try:
                    py_compile.compile(full_path, doraise=True)
                except py_compile.PyCompileError as e:
                    validation_errors.append(f"{file_path}: {e}")
    finally:
        shutil.rmtree(sandbox_dir, ignore_errors=True)

    status = "error" if validation_errors else "success"

    return {
        "files_changed": files_changed,
        "status": status,
        "validation_errors": validation_errors,
    }
