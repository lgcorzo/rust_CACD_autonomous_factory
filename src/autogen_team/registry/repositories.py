"""Registry Repository Interface."""

from abc import ABC, abstractmethod
import typing as T


class RegistryRepository(ABC):
    """Abstract repository for model registry."""

    @abstractmethod
    def register(self, name: str, model_uri: str) -> T.Any:
        """Register a model version."""
        pass

    @abstractmethod
    def promote(self, name: str, version: str, stage: str) -> None:
        """Promote a model version to a stage."""
        pass
