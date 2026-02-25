<<<<<<< HEAD
import os


<<<<<<< HEAD
def safe_join(directory: str, path: str) -> str:
    """Safely join a base directory and a path, preventing traversal.

    Args:
        directory: The base directory.
=======
def safe_join(base: str, path: str) -> str:
    """Safely join a base directory and a path, preventing traversal.

    Args:
        base: The base directory.
>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
        path: The path to join.

    Returns:
        The joined path if safe.

    Raises:
        ValueError: If the path traverses outside the base directory.
    """
<<<<<<< HEAD
    base_dir = os.path.abspath(directory)
    # Join path and resolve relative components (like ..)
    final_path = os.path.abspath(os.path.join(base_dir, path))

    # Ensure the final path is within the base directory
    if os.path.commonpath([base_dir, final_path]) != base_dir:
        raise ValueError(f"Path traversal attempt: {path} is outside of {directory}")
=======
    base_dir = os.path.abspath(base)
    final_path = os.path.abspath(os.path.join(base, path))

    # Use commonpath to check if final_path is inside base_dir
    # os.path.commonpath returns the longest common sub-path
    if os.path.commonpath([base_dir, final_path]) != base_dir:
        raise ValueError(f"Path traversal attempt: {path} is outside of {base}")
>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
=======
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
>>>>>>> origin/sentinel/fix-path-traversal-execute-code-1486026776790124242

    return final_path
