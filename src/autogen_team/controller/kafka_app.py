"""FastAPI and Kafka Service for Predictions with Logging."""

import os
import signal
import threading
import logging
import time
import json
from typing import Any, Dict, Callable

import uvicorn
import pandas as pd
from fastapi import FastAPI, HTTPException
from pydantic import BaseModel

from confluent_kafka import Producer, Consumer, KafkaError

from autogen_team.core.schemas import InputsSchema, Outputs
from autogen_team.io import services, registries
from autogen_team.io.registries import CustomLoader


# Constants
DEFAULT_KAFKA_SERVER = os.getenv("DEFAULT_KAFKA_SERVER", "kafka_server:9092")
DEFAULT_GROUP_ID = os.getenv("DEFAULT_GROUP_ID", "llmops-regression")
DEFAULT_AUTO_OFFSET_RESET = os.getenv("DEFAULT_AUTO_OFFSET_RESET", "earliest")
DEFAULT_INPUT_TOPIC = os.getenv("DEFAULT_INPUT_TOPIC", "llm_input_topic")
DEFAULT_OUTPUT_TOPIC = os.getenv("DEFAULT_OUTPUT_TOPIC", "llm_output_topic")
DEFAULT_FASTAPI_HOST = os.getenv("DEFAULT_FASTAPI_HOST", "127.0.0.1")
DEFAULT_FASTAPI_PORT = int(os.getenv("DEFAULT_FASTAPI_PORT", 8100))
LOGGING_FORMAT = "%(asctime)s - %(levelname)s - %(message)s"


# Configure logging
logging.basicConfig(level=logging.INFO, format=LOGGING_FORMAT)
logger = logging.getLogger(__name__)

# FastAPI App Initialization
app: FastAPI = FastAPI(
    title="Prediction Service API",
    description="A FastAPI service that integrates with Kafka for making predictions.",
    version="1.0.0",
)


# Data Models
class PredictionRequest(BaseModel):
    """Request model for prediction."""

    input_data: Dict[str, Any] = {"input": ["text 1", "text 2"]}

    def model_validate(self) -> Any:
        """Validates the input data against InputsSchema."""
        return InputsSchema.validate(pd.DataFrame([self.input_data]))


class PredictionResponse(BaseModel):
    """Response model for prediction."""

    result: Dict[str, Any] = {"inference": [0.0], "quality": 0.0, "error": ""}


# Core Service Class
class FastAPIKafkaService:
    """Service for deploying a FastAPI application with a Kafka producer and consumer."""

    def __init__(
        self,
        prediction_callback: Callable[[PredictionRequest], PredictionResponse],
        kafka_config: Dict[str, Any],
        input_topic: str,
        output_topic: str,
    ):
        self.server_thread: threading.Thread | None = None
        self.stop_event: threading.Event = threading.Event()
        self.prediction_callback = prediction_callback
        self.kafka_config = kafka_config
        self.input_topic = input_topic
        self.output_topic = output_topic
        self.producer: Producer | None = None
        self.consumer: Consumer | None = None

    def delivery_report(self, err: KafkaError, msg: Any) -> None:
        """Called once for each message produced to indicate delivery result."""
        if err is not None:
            logger.error(f"Message delivery failed: {err}")
        else:
            logger.info(f"Message delivered to {msg.topic()} [{msg.partition()}]")

    def start(self) -> None:
        """Start the FastAPI application and Kafka consumer."""
        self.stop_event.clear()
        self._initialize_kafka_producer()
        self._initialize_kafka_consumer()
        self.server_thread = threading.Thread(target=self._run_server)
        self.server_thread.start()
        time.sleep(2)  # Allow server to start
        threading.Thread(target=self._consume_messages, daemon=True).start()
        logger.info("FastAPI server and Kafka consumer threads started.")

    def _initialize_kafka_producer(self) -> None:
        """Initialize Kafka producer."""
        try:
            self.producer = Producer(self.kafka_config)
            logger.info("Kafka producer initialized")
        except Exception as e:
            logger.error(f"Failed to initialize Kafka producer: {e}")
            raise

    def _initialize_kafka_consumer(self) -> None:
        """Initialize Kafka consumer."""
        self.kafka_config["enable.auto.commit"] = False
        try:
            self.consumer = Consumer(self.kafka_config)
            self.consumer.subscribe([self.input_topic])
            logger.info(f"Kafka consumer subscribed to topic: {self.input_topic}")
        except Exception as e:
            logger.error(f"Failed to initialize Kafka consumer: {e}")
            raise

    def _run_server(self) -> None:
        """Run the FastAPI server."""
        try:
            uvicorn.run(app, host=DEFAULT_FASTAPI_HOST, port=DEFAULT_FASTAPI_PORT, log_level="info")
        except Exception as e:
            logger.error(f"Server error: {e}")

    def _consume_messages(self) -> None:
        """Consume messages from Kafka topic and produce predictions."""
        while not self.stop_event.is_set():
            time.sleep(0.1)
            msg = self._poll_message()
            if msg is None:
                continue
            if msg.error():
                if not self._handle_message_error(msg):
                    break
                continue
            self._process_message(msg)
        self._close_consumer()

    def _poll_message(self) -> Any:
        """Poll message from Kafka consumer."""
        if self.consumer:
            return self.consumer.poll(1.0)
        else:
            logger.error("Kafka consumer is not initialized.")
            return None

    def _handle_message_error(self, msg) -> bool:
        """Handle errors in polled messages."""
        if msg.error().code() == KafkaError._PARTITION_EOF:
            logger.debug("Reached end of partition.")
            return True
        else:
            logger.error(f"Consumer error: {msg.error()}")
            return False

    def _process_message(self, msg) -> None:
        """Process a valid Kafka message."""
        predictionresponse: PredictionResponse = PredictionResponse()
        try:
            kafka_msg = json.loads(msg.value().decode("utf-8"))
            input_obj: PredictionRequest = PredictionRequest()
            input_obj.input_data = kafka_msg["input_data"]
            logger.info(f"kafka Received input  {kafka_msg}")
            prediction_result = self.prediction_callback(input_obj).result
        except json.JSONDecodeError as e:
            error = f"Failed to decode JSON message: {e}. Raw message: {msg.value()}"
            predictionresponse.result["error"] = error
            logger.error(error)
            prediction_result = predictionresponse.result
        except Exception as e:
            error = f"Error during prediction processing: {e}"
            logger.exception(error)
            predictionresponse.result["error"] = error
            prediction_result = predictionresponse.result

        try:
            logger.debug(f"Prediction result: {prediction_result}")
            if self.producer:
                self.producer.produce(
                    self.output_topic,
                    key="prediction",
                    value=json.dumps(prediction_result),
                    callback=self.delivery_report,
                )
                self.producer.flush()
            else:
                logger.error("Kafka producer is not initialized.")
            if self.consumer:
                self.consumer.commit(msg)
        except Exception:
            logger.exception("Error during Kafka production/commit:")

    def _close_consumer(self) -> None:
        """Close the Kafka consumer."""
        if self.consumer:
            self.consumer.close()
        logger.info("Kafka consumer stopped.")

    def stop(self) -> None:
        """Stop the FastAPI application and Kafka consumer."""
        self.stop_event.set()
        if self.consumer:
            self.consumer.close()
            logger.info("Kafka consumer closed.")
        os.kill(os.getpid(), signal.SIGINT)
        logger.info("Service stopped.")


# Global Service Instance
fastapi_kafka_service: FastAPIKafkaService


# FastAPI Endpoints
@app.post(
    "/predict",
    response_model=PredictionResponse,
    summary="Make a Prediction",
    description="This endpoint allows you to submit data for a prediction.",
    tags=["Prediction"],
)
async def predict(request: PredictionRequest) -> PredictionResponse:  # Use global var
    """Endpoint for making predictions via HTTP."""
    global fastapi_kafka_service
    try:
        logger.info(f"Received HTTP prediction request: {request}")
        prediction_result = fastapi_kafka_service.prediction_callback(request)
        logger.info(f"HTTP prediction result: {prediction_result}")
        return prediction_result  # Use the global class
    except Exception as e:
        logger.exception("Error processing HTTP prediction request:")
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/health", summary="Health Check", tags=["System"])
async def health_check() -> Dict[str, str]:
    """Simple health check endpoint to verify that the service is running."""
    return {"status": "healthy"}


def main():
    global fastapi_kafka_service
    # Configuration
    alias_or_version: str | int = "Champion"
    # Initialize Mlflow Service
    mlflow_service = services.MlflowService()
    mlflow_service.start()
    # Load Model
    model_uri = registries.uri_for_model_alias_or_version(
        name=mlflow_service.registry_name, alias_or_version=alias_or_version
    )
    loader = CustomLoader()
    model = loader.load(uri=model_uri)

    # Prediction Callback Function
    def my_prediction_function(input_data: PredictionRequest) -> PredictionResponse:
        predictionresponse: PredictionResponse = PredictionResponse()
        try:
            outputs: Outputs = model.predict(
                inputs=InputsSchema.check(pd.DataFrame(input_data.input_data))
            )
            predictionresponse.result["inference"] = outputs.to_numpy().tolist()
            predictionresponse.result["quality"] = 1
            predictionresponse.result["error"] = None
        except Exception as e:
            predictionresponse.result["inference"] = 0
            predictionresponse.result["quality"] = 0
            predictionresponse.result["error"] = str(e)
        return predictionresponse

    # Kafka Configuration
    kafka_config = {
        "bootstrap.servers": DEFAULT_KAFKA_SERVER,
        "group.id": DEFAULT_GROUP_ID,
        "auto.offset.reset": DEFAULT_AUTO_OFFSET_RESET,
    }
    # Initialize and Start Service
    fastapi_kafka_service = FastAPIKafkaService(
        prediction_callback=my_prediction_function,
        kafka_config=kafka_config,
        input_topic=DEFAULT_INPUT_TOPIC,
        output_topic=DEFAULT_OUTPUT_TOPIC,
    )
    fastapi_kafka_service.start()
    print("FastAPI and Kafka service is running.  Press Ctrl+C to stop.")


# Main Execution
if __name__ == "__main__":
    main()
