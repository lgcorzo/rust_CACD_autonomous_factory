"""MCP Client for connecting to the MCP Server."""

import json
import os
from typing import Any, Dict, Optional

from autogen_team.infrastructure.io.osvariables import Env
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client


class MCPClient:
    """Client for interacting with the MCP Server."""

    def __init__(self) -> None:
        """Initialize the MCP Client."""
        self.env = Env()
        self.session: Optional[ClientSession] = None
        self._exit_stack = None

    async def connect(self) -> None:
        """Connect to the MCP Server via stdio."""
        server_params = StdioServerParameters(
            command="python",
            args=["-m", "autogen_team.application.mcp.mcp_server"],
            env=os.environ.copy(),
        )

        self._client_context = stdio_client(server_params)
        self.read, self.write = await self._client_context.__aenter__()

        self.session = ClientSession(self.read, self.write)
        await self.session.__aenter__()
        await self.session.initialize()

    async def disconnect(self) -> None:
        """Disconnect from the MCP Server."""
        if self.session:
            await self.session.__aexit__(None, None, None)
        if hasattr(self, "_client_context"):
            await self._client_context.__aexit__(None, None, None)

    async def call_tool(self, name: str, arguments: Dict[str, Any]) -> Any:
        """Call a tool on the MCP Server.

        Args:
            name: The name of the tool to call.
            arguments: The arguments to pass to the tool.

        Returns:
            The result of the tool execution.
        """
        if not self.session:
            await self.connect()

        if not self.session:
            raise RuntimeError("Failed to connect to MCP Server")

        result = await self.session.call_tool(name, arguments)

        # Parse the result content
        if result.content and hasattr(result.content[0], "text"):
            try:
                return json.loads(result.content[0].text)
            except json.JSONDecodeError:
                return result.content[0].text
        return result
