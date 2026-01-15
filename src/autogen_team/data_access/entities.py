"""Data Access Domain Entities."""

from dataclasses import dataclass
from typing import Optional


@dataclass
class DatasetDescriptor:
    """Describes a dataset source."""

    name: str
    path: str
    format: str  # "parquet", "json", "csv"
    columns: Optional[list[str]] = None
