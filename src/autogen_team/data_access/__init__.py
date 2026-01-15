"""Data Access Domain - Dataset reading and management."""

from .entities import DatasetDescriptor
from .repositories import DatasetRepository

__all__ = [
    "DatasetDescriptor",
    "DatasetRepository",
]
