import pytest
from unittest.mock import patch, MagicMock
import json
from typing import Generator
import os
import signal

from confluent_kafka import KafkaError


# Assuming the code you provided is in a file named 'app.py'
from autogen_team.controller.kafka_app import (
    FastAPIKafkaService,
    PredictionResponse,
    app,
    DEFAULT_FASTAPI_HOST,
    DEFAULT_FASTAPI_PORT,
)


@pytest.fixture()
def mock_kafka_service() -> (
    Generator[tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock], None, None]
):
    """Fixture to create a mocked FastAPIKafkaService."""
    with (
        patch("autogen_team.controller.kafka_app.Producer") as MockProducer,
        patch("autogen_team.controller.kafka_app.Consumer") as MockConsumer,
        patch("threading.Thread") as MockThread,
        patch("time.sleep") as MockSleep,
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
        yield service, MockProducer, MockConsumer, MockThread, MockSleep


def test_initialization(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test FastAPIKafkaService initialization."""
    service, *_ = mock_kafka_service
    assert service.prediction_callback is not None
    assert service.kafka_config is not None
    assert service.input_topic is not None
    assert service.output_topic is not None
    assert service.producer is None
    assert service.consumer is None


def test_delivery_report(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test delivery report logging."""
    service, *_ = mock_kafka_service
    err = None
    msg = MagicMock()
    msg.topic.return_value = "test_topic"
    msg.partition.return_value = 1

    with patch("autogen_team.controller.kafka_app.logger.info") as mock_logger_info:
        service.delivery_report(err, msg)
        mock_logger_info.assert_called_once()

    err = "Delivery failed"
    with patch("autogen_team.controller.kafka_app.logger.error") as mock_logger_error:
        service.delivery_report(err, msg)
        mock_logger_error.assert_called_once()


def test_start_producer_failure(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test start method when producer initialization fails."""
    service, MockProducer, *_ = mock_kafka_service
    MockProducer.side_effect = Exception("Producer failed")
    with pytest.raises(Exception):
        service.start()


def test_start_consumer_failure(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test start method when consumer initialization fails."""
    service, MockProducer, MockConsumer, *_ = mock_kafka_service
    MockConsumer.side_effect = Exception("Consumer failed")
    with pytest.raises(Exception):
        service.start()


def test_run_server(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the _run_server method."""
    service, *_ = mock_kafka_service
    with patch("autogen_team.controller.kafka_app.uvicorn.run") as mock_uvicorn_run:
        service._run_server()
        mock_uvicorn_run.assert_called_once_with(
            app, host=DEFAULT_FASTAPI_HOST, port=DEFAULT_FASTAPI_PORT, log_level="info"
        )


def test_run_server_failure(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the _run_server method when uvicorn fails."""
    service, *_ = mock_kafka_service
    with patch("autogen_team.controller.kafka_app.uvicorn.run") as mock_uvicorn_run:
        mock_uvicorn_run.side_effect = Exception("Uvicorn failed")
        service._run_server()


def test_consume_messages(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the _consume_messages method."""
    service, *_ = mock_kafka_service
    service.consumer = MagicMock()
    service.producer = MagicMock()
    service.stop_event.is_set = MagicMock(side_effect=[False, True])  # Run once then stop
    service._poll_message = MagicMock(return_value=MagicMock(error=MagicMock(return_value=None)))
    service._process_message = MagicMock()
    service._close_consumer = MagicMock()
    service._handle_message_error = MagicMock(return_value=True)

    service._consume_messages()

    service._poll_message.assert_called_once()
    service._process_message.assert_called_once()
    service._close_consumer.assert_called_once()


def test_consume_messages_with_error(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test _consume_messages handles message errors."""
    service, *_ = mock_kafka_service
    service.consumer = MagicMock()
    service.producer = MagicMock()
    service.stop_event.is_set = MagicMock(side_effect=[False, True])
    error_msg = MagicMock()
    error_msg.error = MagicMock(return_value=MagicMock(code=MagicMock(return_value=1)))
    service._poll_message = MagicMock(return_value=error_msg)
    service._handle_message_error = MagicMock(return_value=False)
    service._process_message = MagicMock()
    service._close_consumer = MagicMock()

    service._consume_messages()

    service._poll_message.assert_called_once()
    service._process_message.assert_not_called()
    service._close_consumer.assert_called_once()


def test_poll_message(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the _poll_message method."""
    service, *_ = mock_kafka_service
    service.consumer = MagicMock()
    service.consumer.poll.return_value = "test_message"
    message = service._poll_message()
    assert message == "test_message"
    service.consumer.poll.assert_called_once_with(1.0)


def test_poll_message_no_consumer(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test _poll_message handles missing consumer."""
    service, *_ = mock_kafka_service
    service.consumer = None
    with patch("autogen_team.controller.kafka_app.logger.error") as mock_logger_error:
        message = service._poll_message()
        assert message is None
        mock_logger_error.assert_called_once()


def test_handle_message_error_partition_eof(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test _handle_message_error handles partition EOF."""
    service, *_ = mock_kafka_service
    msg = MagicMock()
    msg.error.return_value = MagicMock(code=MagicMock(return_value=KafkaError._PARTITION_EOF))
    with patch("autogen_team.controller.kafka_app.logger.debug") as mock_logger_debug:
        result = service._handle_message_error(msg)
        assert result is True
        mock_logger_debug.assert_called_once()


def test_handle_message_error_other_error(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test _handle_message_error handles other Kafka errors."""
    service, *_ = mock_kafka_service
    msg = MagicMock()
    msg.error.return_value = MagicMock(code=MagicMock(return_value=1))
    with patch("autogen_team.controller.kafka_app.logger.error") as mock_logger_error:
        result = service._handle_message_error(msg)
        assert result is False
        mock_logger_error.assert_called_once()


@patch("json.loads")
def test_process_message(
    mock_json_loads: MagicMock,
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the _process_message method."""
    service, *_ = mock_kafka_service
    mock_json_loads.return_value = {"input_data": "test_input"}
    msg = MagicMock()
    msg.value.return_value = b'{"input_data": "test_input"}'
    msg.decode.return_value = '{"input_data": "test_input"}'

    service.producer = MagicMock()
    service.consumer = MagicMock()
    service.prediction_callback.return_value = PredictionResponse(
        result={"inference": [1.0], "quality": 1.0, "error": None}
    )

    service._process_message(msg)

    service.prediction_callback.assert_called_once()
    service.producer.produce.assert_called_once()
    service.producer.flush.assert_called_once()
    service.consumer.commit.assert_called_once_with(msg)


@patch("json.loads")
def test_process_message_json_decode_error(
    mock_json_loads: MagicMock,
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test _process_message handles JSON decoding errors."""
    service, *_ = mock_kafka_service
    mock_json_loads.side_effect = json.JSONDecodeError("Test message", "doc", 0)
    msg = MagicMock()
    msg.value.return_value = b"invalid json"
    msg.decode.return_value = "invalid json"

    service.producer = MagicMock()
    service.consumer = MagicMock()
    with patch("autogen_team.controller.kafka_app.logger.error") as mock_logger_error:
        service._process_message(msg)
        mock_logger_error.assert_called()
    service.prediction_callback.assert_not_called()
    service.producer.produce.assert_called_once()


@patch("json.loads")
def test_process_message_prediction_error(
    mock_json_loads: MagicMock,
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test _process_message handles prediction callback errors."""
    service, *_ = mock_kafka_service
    mock_json_loads.return_value = {"input_data": "test_input"}
    msg = MagicMock()
    msg.value.return_value = b'{"input_data": "test_input"}'
    msg.decode.return_value = '{"input_data": "test_input"}'

    service.producer = MagicMock()
    service.consumer = MagicMock()
    service.prediction_callback.side_effect = Exception("Prediction Failed")
    with patch("autogen_team.controller.kafka_app.logger.exception") as mock_logger_exception:
        service._process_message(msg)
        mock_logger_exception.assert_called()
    # service.prediction_callback.assert_called_once()
    service.producer.produce.assert_called_once()


def test_close_consumer(
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the _close_consumer method."""
    service, *_ = mock_kafka_service
    service.consumer = MagicMock()
    service._close_consumer()
    service.consumer.close.assert_called_once()
    with patch("autogen_team.controller.kafka_app.logger.info") as mock_logger_info:
        service._close_consumer()
        mock_logger_info.assert_called()


@patch("os.kill")
def test_stop(
    mock_os_kill: MagicMock,
    mock_kafka_service: tuple[FastAPIKafkaService, MagicMock, MagicMock, MagicMock, MagicMock],
) -> None:
    """Test the stop method."""
    service, *_ = mock_kafka_service
    service.consumer = MagicMock()
    service.stop()
    service.consumer.close.assert_called_once()
    mock_os_kill.assert_called_once_with(os.getpid(), signal.SIGINT)
    assert service.stop_event.is_set()
    with patch("autogen_team.controller.kafka_app.logger.info") as mock_logger_info:
        service.stop()
        assert service.stop_event.is_set()
        assert mock_logger_info.call_count == 2


def test_main_function() -> None:
    """Test the main function."""
    with (
        patch("autogen_team.controller.kafka_app.services.MlflowService") as MockMlflowService,
        patch(
            "autogen_team.controller.kafka_app.registries.uri_for_model_alias_or_version"
        ) as MockUri,
        patch("autogen_team.controller.kafka_app.CustomLoader") as MockCustomLoader,
        patch("autogen_team.controller.kafka_app.FastAPIKafkaService") as MockFastAPIKafkaService,
        patch("autogen_team.controller.kafka_app.print") as mock_print,
    ):
        # Mock the mlflow service and its methods
        mock_mlflow_service = MagicMock()
        MockMlflowService.return_value = mock_mlflow_service
        mock_mlflow_service.registry_name = "test_registry"

        # Mock the model loader and its methods
        mock_loader = MagicMock()
        MockCustomLoader.return_value = mock_loader
        mock_model = MagicMock()
        mock_loader.load.return_value = mock_model
        mock_model.predict.return_value = MagicMock()

        MockUri.return_value = "test_uri"

        # Call the main function
        from autogen_team.controller.kafka_app import main

        main()

        # Assertions
        MockMlflowService.assert_called_once()
        mock_mlflow_service.start.assert_called_once()
        MockUri.assert_called_once()
        MockCustomLoader.assert_called_once()
        mock_loader.load.assert_called_once_with(uri="test_uri")
        MockFastAPIKafkaService.assert_called_once()
        mock_fastapi_kafka_service = MockFastAPIKafkaService.return_value
        mock_fastapi_kafka_service.start.assert_called_once()
        mock_print.assert_called()
