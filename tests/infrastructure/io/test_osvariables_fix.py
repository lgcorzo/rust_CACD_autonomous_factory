import os
import pytest
from autogen_team.infrastructure.io.osvariables import Env


def test_mcp_port_collision_avoidance():
    # Simulate Kubernetes setting MCP_SERVER_PORT to a URL
    os.environ["MCP_SERVER_PORT"] = "tcp://10.152.183.212:8200"

    # Instantiate Env - this should NOT raise a ValidationError anymore
    # because we renamed the field to mcp_port
    env = Env()

    # Check that mcp_port is still the default (int)
    assert isinstance(env.mcp_port, int)
    assert env.mcp_port == 8200

    # Clean up
    del os.environ["MCP_SERVER_PORT"]


def test_mcp_port_custom_value():
    # Ensure mcp_port can still be set via environment variable
    os.environ["MCP_PORT"] = "9000"

    env = Env()
    assert env.mcp_port == 9000

    # Clean up
    del os.environ["MCP_PORT"]
