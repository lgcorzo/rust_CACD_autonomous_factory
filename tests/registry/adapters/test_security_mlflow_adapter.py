import os
import unittest
from unittest.mock import MagicMock, patch
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver


class TestSecurityLeak(unittest.TestCase):
    @patch.dict(os.environ, {"LITELLM_API_KEY": "super_secret_key"})
    def test_adapter_captures_secret(self):
        mock_model = MagicMock()
        # Initialize the adapter
        adapter = CustomSaver.Adapter(model=mock_model)

        # Check if the secret is captured in model_config
        # This confirms the vulnerability is FIXED

        # model_config should not exist, or if it does, it shouldn't contain the secret
        if hasattr(adapter, "model_config"):
            self.assertNotIn(
                "config", adapter.model_config, "model_config should not contain config dict"
            )
            # If config exists, check for api_key
            if "config" in adapter.model_config:
                self.assertNotEqual(
                    adapter.model_config["config"].get("api_key"),
                    "super_secret_key",
                    "API Key leaked!",
                )
        else:
            print("\nSUCCESS: model_config attribute not found (Vulnerability Fixed)")


if __name__ == "__main__":
    unittest.main()
