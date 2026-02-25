import os
from pathlib import Path

import pytest
from autogen_team.core.security import safe_join


def test_safe_join_valid(tmp_path: Path) -> None:
    """Test safe_join with valid relative paths."""
    base = str(tmp_path)
    path = "file.txt"
    expected = os.path.join(base, path)
    assert safe_join(base, path) == expected


def test_safe_join_nested_valid(tmp_path: Path) -> None:
    """Test safe_join with nested valid paths."""
    base = str(tmp_path)
    path = "subdir/file.txt"
    expected = os.path.join(base, "subdir", "file.txt")
    assert safe_join(base, path) == expected


def test_safe_join_traversal(tmp_path: Path) -> None:
    """Test safe_join prevents directory traversal."""
    base = str(tmp_path)
    # Attempt to go up one level
    path = "../secret.txt"
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


def test_safe_join_traversal_complex(tmp_path: Path) -> None:
    """Test safe_join prevents complex traversal."""
    base = str(tmp_path)
    path = "subdir/../../secret.txt"
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


def test_safe_join_absolute_escape(tmp_path: Path) -> None:
    """Test safe_join prevents absolute paths escaping base."""
    base = str(tmp_path)
    # Try to access /etc/passwd or similar
    path = "/etc/passwd"
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


def test_safe_join_directory_prefix_edge_case(tmp_path: Path) -> None:
    """Test that safe_join handles directory prefix edge cases correctly.

    e.g. base="/tmp/foo", path="../foobar/baz" should fail even though "/tmp/foobar" starts with "/tmp/foo".
    """
    base = str(tmp_path / "foo")
    os.makedirs(base)

    # Create a sibling directory that shares the prefix
    sibling = str(tmp_path / "foobar")
    os.makedirs(sibling)

    # Try to access the sibling
    path = "../foobar/baz"

    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)
