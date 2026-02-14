# Implementation Plan - MCP Server Implementation (DA-7)

## Goal Description

Implement the **Model Context Protocol (MCP)** server and its associated tools to empower the Autonomous Agents (Planner, Coder, Tester, Reviewer) with actual capabilities. Currently, these agents use mocked tool calls.

## User Review Required

> [!IMPORTANT]
> This plan involves replacing mocked agent logic with real MCP tool calls.
> **Dependency**: Requires valid credentials/connections for underlying services (e.g., GitHub, Database) if the tools interact with them directly.

## Proposed Changes

### MCP Server Layer

#### [MODIFY] [mcp_server.py](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/src/autogen_team/application/mcp/mcp_server.py)

- Ensure server is configured to expose the required tools.
- Verify initialization and connection handling.

#### [NEW/MODIFY] Tools in `src/autogen_team/application/mcp/tools/`

Implement or refactor the following tools:

1.  **`plan_mission.py`**: Accepts a high-level goal, returns a structured plan (JSON).
2.  **`execute_code.py`**: Accepts code/file changes, applies them to the repository (using file system or Git).
3.  **`run_tests.py`**: triggers `pytest` or similar commands and returns results.
4.  **`security_review.py`**: Analyzes code diffs for security issues (mocked for now or using simple regex/patterns).
5.  **`create_pull_request.py`**: (Optional for DA-7, but good to have) usage of GitHub API.

### Agent Layer

#### [MODIFY] Agent Classes

Update the following agents to use the `mcp-sdk` (or internal tool logic if running in same process) to call the above tools:

- `src/autogen_team/application/agents/planner_agent.py` -> calls `plan_mission`
- `src/autogen_team/application/agents/coder_agent.py` -> calls `execute_code`
- `src/autogen_team/application/agents/tester_agent.py` -> calls `run_tests`
- `src/autogen_team/application/agents/reviewer_agent.py` -> calls `security_review`

### Infrastructure Layer

#### [NEW] [mcp_client.py](file:///mnt/F024B17C24B145FE/Repos/llmops-python-package/src/autogen_team/infrastructure/client/mcp_client.py)

- Implement a client capability to connect to the MCP Server (stdio or SSE).
- Provide method `call_tool(name, args)` to be used by Agents.

## Verification Plan

### Automated Tests

- Create/Update unit tests for each tool function.
- Create an integration test where an agent calls a tool and verifies the output.

### Manual Verification

- Start MCP server locally.
- Use an MCP inspector or simple client script to list tools and call them manually.
