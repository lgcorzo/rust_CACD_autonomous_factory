"""Security utilities for the application."""

import os
import typing as T


def safe_join(base: str, *paths: str) -> str:
    """Safely join paths, ensuring the result is within the base directory.

    Args:
        base (str): The base directory.
        *paths (str): Paths to join.

    Returns:
        str: The joined path.

    Raises:
        ValueError: If the resolved path is outside the base directory.
    """
    base = os.path.abspath(base)
    final_path = os.path.abspath(os.path.join(base, *paths))

    # Ensure the final path starts with the base path
    if os.path.commonpath([base, final_path]) != base:
        raise ValueError(f"Path traversal detected: {final_path} is not within {base}")

    return final_path
