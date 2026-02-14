# Workflow DSL & MCP Client Implementation — Task Checklist

## Phase 1: Infrastructure - MCP Client
- [x] Create `src/autogen_team/infrastructure/client/mcp_client.py` to handle MCP connections
- [ ] Unit tests for `mcp_client.py`

## Phase 2: Agent Integration
- [x] Update `PlannerAgent` to use `MCPClient` for `plan_mission`
- [x] Update `CoderAgent` to use `MCPClient` for `execute_code`
- [x] Update `TesterAgent` to use `MCPClient` for `run_tests`
- [x] Update `ReviewerAgent` to use `MCPClient` for `security_review`

## Phase 3: Workflow DSL Implementation
- [x] Refactor `autonomous_mission.py` to use `HatchetService`
- [x] Create `Scripts/run_hatchet_worker.py` to run the worker
- [x] Create `Scripts/trigger_mission.py` to trigger the workflow

## Phase 4: CI Error Resolution (Ruff, Mypy, Pytest)
    [x] Run ruff and fix style errors
    [x] Run mypy and fix type errors in agents, workflows, and tests
    [x] Fix ImportErrors in tests/conftest.py (agent_framework update)
    [x] Align hatchet_workflows.py with new SDK DSL
    [x] Run full test suite and ensure all tests pass
- [x] Verify `AutonomousMissionWorkflow` end-to-end with real MCP server (Verified via Hatchet worker logs)
