"""Registry Domain Entities."""

from dataclasses import dataclass
from typing import Optional


@dataclass
class ModelVersion:
    """Represents a registered model version."""

    name: str
    version: str
    model_uri: str
    stage: str = "None"  # "Staging", "Production", "Archived"


@dataclass
class ModelInfo:
    """Represents model metadata."""

    model_uri: str
    run_id: Optional[str] = None
