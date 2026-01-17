"""Model Repository Interface."""

import typing as T
from abc import ABC, abstractmethod


class ModelRepository(ABC):
    """Abstract repository for model persistence."""

    @abstractmethod
    def save(self, model: T.Any, path: str) -> None:
        """Save model to storage."""
        pass

    @abstractmethod
    def load(self, path: str) -> T.Any:
        """Load model from storage."""
        pass
