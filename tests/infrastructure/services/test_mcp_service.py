"""Tests for MCPService."""

import pytest
import httpx
from unittest.mock import MagicMock, patch
from autogen_team.infrastructure.services.mcp_service import MCPService



@pytest.fixture
def mcp_service():
    """Fixture to provide an MCPService instance with default config."""
    # Patch the class-level env attribute since it's used in Field default_factory
    mock_env = MagicMock()
    mock_env.litellm_api_base = "http://litellm"
    mock_env.litellm_api_key = "key"
    mock_env.litellm_model = "gpt-4"
    mock_env.r2r_base_url = "http://r2r"
    mock_env.mcp_prompts_path = "/tmp/prompts.yaml"
    
    with patch.object(MCPService, "env", mock_env):
        service = MCPService(
            litellm_api_base="http://litellm",
            litellm_api_key="key",
            litellm_model="gpt-4",
            r2r_base_url="http://r2r",
            prompts_path="/tmp/prompts.yaml"
        )
        return service


def test_mcp_service_start(mcp_service):
    """Test MCPService.start initializes clients and config."""
    with patch("autogen_team.infrastructure.services.mcp_service.litellm") as mock_litellm:
        mcp_service.start()
        
        assert mock_litellm.api_base == "http://litellm"
        assert mock_litellm.api_key == "key"
        assert mcp_service._r2r_client is not None
        assert isinstance(mcp_service._r2r_client, httpx.AsyncClient)



def test_mcp_service_load_prompts_not_found():
    """Test _load_prompts when file doesn't exist (line 60)."""
    service = MCPService(prompts_path="/non/existent/path")
    service._load_prompts()
    assert service._prompts == {}


def test_mcp_service_get_prompt_lazy_load(mcp_service):
    """Test get_prompt triggers lazy loading."""
    with patch("autogen_team.infrastructure.services.mcp_service.MCPService._load_prompts") as mock_load:
        # We can't easily reset _prompts on a frozen instance, 
        # but the class-level patch will capture the call when get_prompt is called if _prompts is None.
        # Let's create a fresh one where _prompts IS None.
        service = MCPService()
        service.get_prompt("tool")
        mock_load.assert_called_once()


def test_mcp_service_stop(mcp_service):
    """Test MCPService.stop (lines 72-74)."""
    # Since it's frozen, we can't set _r2r_client directly.
    # But start() sets it.
    mcp_service.start()
    assert mcp_service._r2r_client is not None
    mcp_service.stop()
    assert mcp_service._r2r_client is None


def test_mcp_service_r2r_client_property(mcp_service):
    """Test r2r_client property auto-starts (lines 79-83)."""
    with patch("autogen_team.infrastructure.services.mcp_service.MCPService.start") as mock_start:
        # We need a service where _r2r_client is None
        service = MCPService()
        
        # Mock start to do nothing (it's already mocked)
        # But we need to bypass the _r2r_client is None check or mock it too.
        # Actually, if we mock start, it won't set _r2r_client, so it will raise RuntimeError 
        # after the patch if we are not careful.
        
        # Let's just test that start is called.
        with pytest.raises(RuntimeError): # Because start didn't actually set the client
            _ = service.r2r_client
        mock_start.assert_called_once()


def test_mcp_service_r2r_client_failure():
    """Test r2r_client property raises error if start fails (line 82)."""
    service = MCPService()
    with patch("autogen_team.infrastructure.services.mcp_service.MCPService.start"):
        with pytest.raises(RuntimeError, match="R2R client failed to initialize"):
            _ = service.r2r_client
