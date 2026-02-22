import os
import pytest
import typing as T
import pathlib
from autogen_team.core.security import safe_join


def test_safe_join_valid(tmp_path: pathlib.Path) -> None:
    """Test safe_join with valid paths."""
    base = str(tmp_path)
    path = "file.txt"
    expected = os.path.join(base, path)
    assert safe_join(base, path) == expected


def test_safe_join_subdir(tmp_path: pathlib.Path) -> None:
    """Test safe_join with subdirectory."""
    base = str(tmp_path)
    path = "subdir/file.txt"
    expected = os.path.join(base, "subdir", "file.txt")
    assert safe_join(base, path) == expected


def test_safe_join_traversal(tmp_path: pathlib.Path) -> None:
    """Test safe_join with path traversal."""
    base = str(tmp_path)
    path = "../outside.txt"
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


def test_safe_join_absolute_traversal(tmp_path: pathlib.Path) -> None:
    """Test safe_join with absolute path outside base."""
    base = str(tmp_path)
    # Use a path that is definitely outside tmp_path
    path = "/etc/passwd"
    # But wait, on some systems /tmp might be symlinked.
    # Let's use a path relative to root but different tree

    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


def test_safe_join_nested_traversal(tmp_path: pathlib.Path) -> None:
    """Test safe_join with nested traversal."""
    base = str(tmp_path)
    path = "subdir/../../outside.txt"
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


def test_safe_join_prefix_collision(tmp_path: pathlib.Path) -> None:
    """Test directory name prefix collision."""
    # e.g. base="/tmp/foo", path="../foobar/file" -> "/tmp/foobar/file"
    # This shouldn't happen with commonpath but good to test.
    base = str(tmp_path / "foo")
    os.makedirs(base)

    # "foobar" starts with "foo" but is a sibling directory
    path = "../foobar/file.txt"

    # Depending on how it's resolved, this might look like it's inside if string matching is used.
    # But commonpath handles this correctly.
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)
