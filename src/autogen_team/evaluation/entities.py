"""Evaluation Domain Entities."""

from dataclasses import dataclass


@dataclass
class MetricResult:
    """Represents a metric evaluation result."""

    name: str
    value: float
    greater_is_better: bool = True
