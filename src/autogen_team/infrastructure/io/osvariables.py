from typing import Dict, Type

from pydantic_settings import BaseSettings


class Singleton(object):
    _instances: Dict[Type["Singleton"], "Singleton"] = {}

    def __new__(cls: Type["Singleton"], *args: object, **kwargs: object) -> "Singleton":
        if cls not in cls._instances:
            cls._instances[cls] = super().__new__(cls, *args, **kwargs)  # Corrected super() call
        return cls._instances[cls]


class Env(Singleton, BaseSettings):
    mlflow_tracking_uri: str = "./mlruns"
    mlflow_registry_uri: str = "./mlruns"
    mlflow_experiment_name: str = "autogen_team"
    mlflow_registered_model_name: str = "autogen_team"

    # S3
    aws_access_key_id: str = ""
    aws_secret_access_key: str = ""
    mlflow_s3_endpoint_url: str = ""
    mlflow_s3_ignore_tls: bool = False

    # Hatchet
    hatchet_client_token: str = ""
    hatchet_namespace: str = "autogen_team"

    # MCP / LiteLLM
    litellm_api_base: str = "http://litellm.llm-apps.svc.cluster.local:4000/v1"
    litellm_api_key: str = ""
    litellm_model: str = "gemini/gemini-2.5-pro"

    # R2R RAG
    r2r_base_url: str = "http://r2r.knowledge.svc.cluster.local:7272"

    # MCP Server
    mcp_server_host: str = "0.0.0.0"  # nosec B104
    mcp_server_port: int = 8200
    mcp_prompts_path: str = "confs/mcp_prompts.yaml"

    class Config:
        case_sensitive = False  # Optional: make env var lookup case-insensitive
        env_file = ".env"  # Enable reading from .env file
        env_file_encoding = "utf-8"
        extra = "allow"
