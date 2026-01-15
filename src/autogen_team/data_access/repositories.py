"""Data Access Repository Interface."""

from abc import ABC, abstractmethod

import pandas as pd


class DatasetRepository(ABC):
    """Abstract repository for dataset access."""

    @abstractmethod
    def read(self) -> pd.DataFrame:
        """Read dataset into DataFrame."""
        pass
