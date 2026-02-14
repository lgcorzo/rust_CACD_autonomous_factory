# Workflow DSL & MCP Client Implementation Plan

## Goal Description
Implement the client-side infrastructure to allow Agents to communicate with the MCP Server, effectively replacing mocked logic with real tool execution.

## User Review Required
> [!IMPORTANT]
> This change introduces a runtime dependency on the running MCP Server.

## Proposed Changes

### Infrastructure Layer
#### [NEW] [mcp_client.py](file:///home/lgcorzo/llmops-python-package/src/autogen_team/infrastructure/client/mcp_client.py)
- Implement `MCPClient` using `mcp` library (stdio or SSE).
- Provide `call_tool(tool_name, arguments)` method.

### Agent Layer
#### [MODIFY] [planner_agent.py](file:///home/lgcorzo/llmops-python-package/src/autogen_team/application/agents/planner_agent.py)
- Inject `MCPClient`.
- Call `plan_mission` tool.

#### [MODIFY] [coder_agent.py](file:///home/lgcorzo/llmops-python-package/src/autogen_team/application/agents/coder_agent.py)
- Inject `MCPClient`.
- Call `execute_code` tool.

#### [MODIFY] [tester_agent.py](file:///home/lgcorzo/llmops-python-package/src/autogen_team/application/agents/tester_agent.py)
- Inject `MCPClient`.
- Call `run_tests` tool.

#### [MODIFY] [reviewer_agent.py](file:///home/lgcorzo/llmops-python-package/src/autogen_team/application/agents/reviewer_agent.py)
- Inject `MCPClient`.
- Call `security_review` tool.

## Phase 3: Workflow DSL Implementation
### Application Layer
#### [MODIFY] [autonomous_mission.py](file:///home/lgcorzo/llmops-python-package/src/autogen_team/application/workflows/autonomous_mission.py)
- Refactor to use `HatchetService`.
- Implement dynamic fan-out for tasks if supported, or sequential execution.
- Ensure context propagation.

## Verification Plan
### Automated Tests
- [x] Unit tests for `MCPClient` (Verified via `verify_agent_mcp.py`).
- [ ] Integration test running the full `AutonomousMissionWorkflow` with the local MCP server (Blocked by Hatchet connectivity).
