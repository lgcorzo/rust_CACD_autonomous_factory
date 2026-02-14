from typing import Dict, Any, List


class CoderAgent:
    """
    Agent responsible for executing coding tasks.
    Currently mocks the MCP tool call.
    """

    def __init__(self):
        pass

    async def execute_task(self, task: Dict[str, Any]) -> Dict[str, Any]:
        """
        Calls the `execute_code` tool via MCP (mocked for now).
        """
        print(f"[CoderAgent] Executing task: {task['description']}")

        # TODO: Replace with actual MCP tool call
        # result = await mcp_client.call_tool("execute_code", {"instruction": task['description'], ...})

        return {
            "task_id": task["id"],
            "status": "completed",
            "diff": "diff --git a/src/api/handler.py b/src/api/handler.py\n+ def handle_request():\n+     pass",
            "file_changes": task.get("files", []),
        }
