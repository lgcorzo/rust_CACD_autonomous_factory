import json
import os
import tempfile
from unittest.mock import MagicMock, patch

import pytest
from autogen_team.application.mcp.tools.execute_code import execute_code
from autogen_team.application.mcp.tools.run_tests import run_tests


@pytest.mark.asyncio
async def test_execute_code_path_traversal() -> None:
    # Setup
    target_file = "/tmp/autogen_team_pwned.txt"
    if os.path.exists(target_file):
        os.remove(target_file)

    # The vulnerability: ../../../../../../../../../tmp/autogen_team_pwned.txt
    malicious_path = "../../../../../../../../../../tmp/autogen_team_pwned.txt"

    mock_response = MagicMock()
    mock_response.choices = [MagicMock()]
    mock_response.choices[0].message.content = json.dumps(
        {"files_changed": [{"path": malicious_path, "action": "create", "content": "PWNED"}]}
    )

    with patch("autogen_team.application.mcp.tools.execute_code.MCPService") as MockMCPService:
        MockMCPService.return_value.get_prompt.return_value = "system prompt"
        MockMCPService.return_value.litellm_model = "gpt-4o"
        MockMCPService.return_value.litellm_api_base = "http://localhost"
        MockMCPService.return_value.litellm_api_key = "fake"

        with patch("litellm.acompletion", return_value=mock_response):
            await execute_code({"name": "test task"}, "/tmp")

            if os.path.exists(target_file):
                content = open(target_file).read()
                os.remove(target_file)
                if content == "PWNED":
                    pytest.fail(f"VULNERABILITY EXPLOITED: File written to {target_file}")


@pytest.mark.asyncio
async def test_run_tests_path_traversal() -> None:
    # Setup
    target_file = "/tmp/autogen_team_pwned_run_tests.txt"
    if os.path.exists(target_file):
        os.remove(target_file)

    malicious_path = "../../../../../../../../../../tmp/autogen_team_pwned_run_tests.txt"

    changes = {"files_changed": [{"path": malicious_path, "action": "create", "content": "PWNED"}]}

    # Use a safe temp dir as workspace
    with tempfile.TemporaryDirectory() as workspace_dir:
        await run_tests(changes, workspace_dir)

    if os.path.exists(target_file):
        content = open(target_file).read()
        os.remove(target_file)
        if content == "PWNED":
            pytest.fail(f"VULNERABILITY EXPLOITED: File written to {target_file}")
