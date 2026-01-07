import os

from confluent_kafka.admin import AdminClient, NewTopic

# Load env vars implicitly or explicitly
env_kafka_server = os.getenv("DEFAULT_KAFKA_SERVER", "my-kafka-cluster.confluent.svc.cluster.local:9092")
print(f"Connecting to Kafka at {env_kafka_server}")

admin_client = AdminClient({"bootstrap.servers": env_kafka_server})

topics_to_create = ["llm_input_topic", "llm_output_topic"]
new_topics = [NewTopic(topic, num_partitions=1, replication_factor=1) for topic in topics_to_create]

fs = admin_client.create_topics(new_topics)

for topic, f in fs.items():
    try:
        f.result()  # The result itself is None
        print(f"Topic {topic} created")
    except Exception as e:
        print(f"Failed to create topic {topic}: {e}")
