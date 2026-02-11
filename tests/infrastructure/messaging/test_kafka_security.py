import json
from typing import Generator
from unittest.mock import MagicMock, patch

import pytest
from autogen_team.infrastructure.messaging.kafka_app import (
    FastAPIKafkaService,
    PredictionResponse,
)


@pytest.fixture()
def mock_kafka_service() -> Generator[FastAPIKafkaService, None, None]:
    """Fixture to create a mocked FastAPIKafkaService."""
    with (
        patch("autogen_team.infrastructure.messaging.kafka_app.Producer") as MockProducer,
        patch("autogen_team.infrastructure.messaging.kafka_app.Consumer") as MockConsumer,
        patch("threading.Thread"),
        patch("time.sleep"),
    ):
        mock_producer = MagicMock()
        MockProducer.return_value = mock_producer
        mock_consumer = MagicMock()
        MockConsumer.return_value = mock_consumer

        prediction_callback = MagicMock(return_value=PredictionResponse())
        kafka_config = {
            "bootstrap.servers": "kafka_server:9092",
            "group.id": "test_group",
            "auto.offset.reset": "earliest",
        }
        input_topic = "test_input_topic"
        output_topic = "test_output_topic"

        service = FastAPIKafkaService(
            prediction_callback=prediction_callback,
            kafka_config=kafka_config,
            input_topic=input_topic,
            output_topic=output_topic,
        )
        yield service


def test_process_message_generic_error_on_exception(
    mock_kafka_service: FastAPIKafkaService,
) -> None:
    """Test that _process_message returns a generic error message on exception."""
    service = mock_kafka_service

    msg = MagicMock()
    msg.value.return_value = b'{"input_data": "sensitive_data"}'
    msg.decode.return_value = '{"input_data": "sensitive_data"}'

    service.producer = MagicMock()
    service.consumer = MagicMock()

    # Simulate an exception in the callback with sensitive info
    service.prediction_callback.side_effect = Exception("Sensitive internal error: /path/to/secret")

    # Patch json.loads ONLY within the context of _process_message execution if needed,
    # OR better, since we are using MagicMock for msg, we can just rely on real json.loads working if msg.value() is valid JSON.
    # msg.value() returns bytes b'{"input_data": "sensitive_data"}'. Real json.loads will parse this correctly.
    # So we don't need to patch json.loads at all!

    with patch("autogen_team.infrastructure.messaging.kafka_app.logger") as mock_logger:
        service._process_message(msg)

        # Verify the error logged contains the detail (for debugging)
        mock_logger.exception.assert_called()
        args, _ = mock_logger.exception.call_args
        assert "Sensitive internal error" in args[0]

    # Verify the produced message contains generic error
    service.producer.produce.assert_called_once()
    args, kwargs = service.producer.produce.call_args
    value_json = json.loads(kwargs["value"].decode("utf-8"))

    assert value_json["error"] == "Internal processing error"
    assert "Sensitive internal error" not in value_json["error"]


def test_process_message_no_pii_logging(
    mock_kafka_service: FastAPIKafkaService,
) -> None:
    """Test that _process_message does not log raw input data."""
    service = mock_kafka_service
    sensitive_input = {"input_data": "my_secret_password"}

    msg = MagicMock()
    msg.value.return_value = json.dumps(sensitive_input).encode("utf-8")
    # No need to mock decode, real bytes decode works. But wait, msg is a Mock.
    # msg.value() returns the bytes. The code calls msg.value().decode("utf-8").
    # If msg.value() returns a MagicMock, decode might fail or return another mock.
    # So we should set return_value of msg.value() to actual bytes.

    service.producer = MagicMock()
    service.consumer = MagicMock()
    service.prediction_callback.return_value = PredictionResponse()

    with patch("autogen_team.infrastructure.messaging.kafka_app.logger") as mock_logger:
        service._process_message(msg)

        # Check all info calls
        found_input_log = False
        for call in mock_logger.info.call_args_list:
            args, _ = call
            if "kafka Received input" in str(args[0]):
                found_input_log = True
                assert "my_secret_password" not in str(args[0])

        assert found_input_log
