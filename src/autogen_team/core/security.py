"""Security utilities for the application."""

import os


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
    base_dir = os.path.abspath(base)
    final_path = os.path.abspath(os.path.join(base_dir, *paths))

    # Ensure the final path starts with the base path
    if os.path.commonpath([base_dir, final_path]) != base_dir:
        raise ValueError(f"Path traversal detected: {final_path} is not within {base_dir}")

    return final_path
