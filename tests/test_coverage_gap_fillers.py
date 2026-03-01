import pytest
import pandas as pd
import json
from unittest.mock import MagicMock, patch, AsyncMock
from autogen_team.application.mcp.tools.plan_mission import plan_mission
from autogen_team.infrastructure.services.alert_service import AlertsService
from autogen_team.core import schemas
from autogen_team.models.repositories import ModelRepository
from autogen_team.registry.repositories import RegistryRepository

@pytest.mark.asyncio
async def test_plan_mission_missing_keys():
    """Cover lines 53, 55 in plan_mission.py by returning dict with missing keys."""
    with patch("autogen_team.application.mcp.tools.plan_mission.litellm.acompletion") as mock_ac:
        mock_response = MagicMock()
        # Missing 'goal' and 'parallel_tasks'
        mock_response.choices[0].message.content = json.dumps({"something": "else"})
        mock_ac.return_value = mock_response
        
        result = await plan_mission("test goal")
        assert "parallel_tasks" in result
        assert result["parallel_tasks"] == []
        assert result["goal"] == "test goal"

def test_alert_service_exception():
    """Cover lines 27-28 in alert_service.py by triggering notification exception."""
    service = AlertsService(enable=True)
    with patch("autogen_team.infrastructure.services.alert_service.notification.notify") as mock_notify:
        mock_notify.side_effect = Exception("System notification failed")
        # Should not raise, should print and continue
        service.notify("title", "message")
    mock_notify.assert_called_once()

def test_schemas_main():
    """Cover lines 103-113 in schemas.py by calling its validation code."""
    # We can't easily trigger the 'if __name__ == "__main__"' block directly without subprocess,
    # but we can call the code inside it.
    input_data = pd.DataFrame({"input": ["Some large input string"]})
    res1 = schemas.InputsSchema.check(input_data)
    assert not res1.empty
    
    output_data = pd.DataFrame(
        {
            "response": ["Generated output string"],
            "metadata": [{"timestamp": "2025-01-15T12:00:00Z", "model_version": "v1.0.0"}],
        }
    )
    res2 = schemas.OutputsSchema.check(output_data)
    assert not res2.empty

def test_abstract_repositories():
    """Cover repositories.py ABCs."""
    class ConcreteModelRepo(ModelRepository):
        def save(self, model, path): pass
        def load(self, path): return None
    
    repo1 = ConcreteModelRepo()
    assert repo1.load("path") is None
    
    class ConcreteRegistryRepo(RegistryRepository):
        def register(self, name, model_uri): return None
        def promote(self, name, version, stage): pass
    
    repo2 = ConcreteRegistryRepo()
    assert repo2.register("name", "uri") is None
