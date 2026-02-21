import os


def safe_join(directory: str, path: str) -> str:
    """Safely join a base directory and a path, preventing traversal.

    Args:
        directory: The base directory.
        path: The path to join.

    Returns:
        The joined path if safe.

    Raises:
        ValueError: If the path traverses outside the base directory.
    """
    base_dir = os.path.abspath(directory)
    # Join path and resolve relative components (like ..)
    final_path = os.path.abspath(os.path.join(base_dir, path))

    # Ensure the final path is within the base directory
    if os.path.commonpath([base_dir, final_path]) != base_dir:
        raise ValueError(f"Path traversal attempt: {path} is outside of {directory}")

    return final_path
