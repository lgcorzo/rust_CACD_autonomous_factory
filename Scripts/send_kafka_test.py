import json
import os
import time
import uuid

from confluent_kafka import Consumer, Producer

# Configuration
KAFKA_SERVER = os.getenv("DEFAULT_KAFKA_SERVER", "my-kafka-cluster.confluent.svc.cluster.local:9092")
INPUT_TOPIC = "llm_input_topic"
OUTPUT_TOPIC = "llm_output_topic"


def delivery_report(err, msg):
    if err is not None:
        print(f"Message delivery failed: {err}")
    else:
        print(f"Message delivered to {msg.topic()} [{msg.partition()}]")


def main():
    print(f"Connecting to Kafka broker: {KAFKA_SERVER}")

    # Initialize Consumer BEFORE sending to ensure we don't miss the message
    # Use a unique group ID to ensure we get the latest message and don't share offsets weirdly during testing
    consumer_conf = {
        "bootstrap.servers": KAFKA_SERVER,
        "group.id": f"test_script_{uuid.uuid4()}",
        "auto.offset.reset": "earliest",
    }
    consumer = Consumer(consumer_conf)
    consumer.subscribe([OUTPUT_TOPIC])
    print(f"Subscribed to {OUTPUT_TOPIC}")

    # Initialize Producer
    producer = Producer({"bootstrap.servers": KAFKA_SERVER})

    # Prepare message payload
    payload = {"input_data": [{"input": "how is cristiano ronaldo"}]}

    json_payload = json.dumps(payload)
    print(f"Sending message: {json_payload}")

    producer.produce(INPUT_TOPIC, key=b"test_key", value=json_payload.encode("utf-8"), callback=delivery_report)
    producer.flush()

    print(f"Waiting for response...")
    start_time = time.time()
    try:
        while True:
            msg = consumer.poll(1.0)
            if msg is None:
                if time.time() - start_time > 30:  # Timeout after 30s
                    print("Timeout waiting for response.")
                    break
                continue
            if msg.error():
                print("Consumer error: {}".format(msg.error()))
                continue

            print("------------------------------------------------")
            print(f"Received response: {msg.value().decode('utf-8')}")
            print("------------------------------------------------")
            break
    except KeyboardInterrupt:
        pass
    finally:
        consumer.close()


if __name__ == "__main__":
    main()
