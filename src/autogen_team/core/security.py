import os


def safe_join(base: str, path: str) -> str:
    """Safely join a base directory and a path, preventing traversal.

    Args:
        base: The base directory.
        path: The path to join.

    Returns:
        The joined path if safe.

    Raises:
        ValueError: If the path traverses outside the base directory.
    """
    base_dir = os.path.abspath(base)
    final_path = os.path.abspath(os.path.join(base, path))

    # Use commonpath to check if final_path is inside base_dir
    # os.path.commonpath returns the longest common sub-path
    if os.path.commonpath([base_dir, final_path]) != base_dir:
        raise ValueError(f"Path traversal attempt: {path} is outside of {base}")

    return final_path
