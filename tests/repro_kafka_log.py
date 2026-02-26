
import unittest
import json
import logging
from unittest.mock import MagicMock, patch
from autogen_team.infrastructure.messaging.kafka_app import FastAPIKafkaService

class TestKafkaAppLogging(unittest.TestCase):
    def test_log_raw_message_on_json_error(self):
        # Given
        service = FastAPIKafkaService(
            prediction_callback=MagicMock(),
            producer_config={},
            consumer_config={},
            input_topic="in",
            output_topic="out"
        )

        mock_msg = MagicMock()
        # Malformed JSON containing sensitive info
        raw_value = b'{"input_data": "sensitive_password", '
        mock_msg.value.return_value = raw_value
        mock_msg.error.return_value = None

        # Capture logs
        with patch('autogen_team.infrastructure.messaging.kafka_app.logger') as mock_logger:
            # When
            service._process_message(mock_msg)

            # Then
            # Verify error log was called
            self.assertTrue(mock_logger.error.called)

            # Check if any error log contains the raw value
            found_sensitive_log = False
            for call in mock_logger.error.call_args_list:
                args, _ = call
                log_message = str(args[0])
                if str(raw_value) in log_message:
                    found_sensitive_log = True
                    break

            self.assertFalse(found_sensitive_log, "Found sensitive data in error logs")

if __name__ == '__main__':
    unittest.main()
