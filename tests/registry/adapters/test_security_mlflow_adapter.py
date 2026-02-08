import os
import unittest
from unittest.mock import MagicMock
from autogen_team.registry.adapters.mlflow_adapter import CustomSaver


class TestSecurityMlflowAdapter(unittest.TestCase):
    def test_no_secret_leakage_in_adapter_init(self) -> None:
        """
        Test that CustomSaver.Adapter does not capture environment variables
        (secrets) in its __init__ method, which would be pickled into the model artifact.
        """
        # Setup fake secret
        fake_secret = "sk-leak-test-12345"
        os.environ["LITELLM_API_KEY"] = fake_secret

        # Mock the model
        mock_model = MagicMock()

        # Initialize adapter
        adapter = CustomSaver.Adapter(model=mock_model)

        # Check for secret in instance attributes
        found_secret = False
        if hasattr(adapter, "model_config"):
            # Check deep inside model_config
            import json

            config_str = json.dumps(adapter.model_config)
            if fake_secret in config_str:
                found_secret = True

        self.assertFalse(
            found_secret, "CRITICAL: CustomSaver.Adapter captured LITELLM_API_KEY in model_config!"
        )

        # Double check __dict__
        for key, value in adapter.__dict__.items():
            if isinstance(value, str) and fake_secret in value:
                self.fail(f"Secret found in attribute {key}")
            if isinstance(value, dict):
                import json

                try:
                    if fake_secret in json.dumps(value):
                        self.fail(f"Secret found in dictionary attribute {key}")
                except TypeError:
                    pass


if __name__ == "__main__":
    unittest.main()
