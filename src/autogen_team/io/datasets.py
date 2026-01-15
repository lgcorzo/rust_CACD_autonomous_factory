"""Legacy IO Datasets - Re-export from data_access."""

from autogen_team.data_access.adapters import (
    Reader,
    ParquetReader,
    ReaderKind,
    Writer,
    ParquetWriter,
    WriterKind,
    Lineage,
)

__all__ = [
    "Reader",
    "ParquetReader",
    "ReaderKind",
    "Writer",
    "ParquetWriter",
    "WriterKind",
    "Lineage",
]
