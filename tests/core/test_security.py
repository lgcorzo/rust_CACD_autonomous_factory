import os
<<<<<<< HEAD
from pathlib import Path

import pytest
from autogen_team.core.security import safe_join


def test_safe_join_valid(tmp_path: Path) -> None:
    """Test safe_join with valid relative paths."""
=======
import pytest
import typing as T
import pathlib
from autogen_team.core.security import safe_join


def test_safe_join_valid(tmp_path: pathlib.Path) -> None:
    """Test safe_join with valid paths."""
>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
    base = str(tmp_path)
    path = "file.txt"
    expected = os.path.join(base, path)
    assert safe_join(base, path) == expected


<<<<<<< HEAD
def test_safe_join_nested_valid(tmp_path: Path) -> None:
    """Test safe_join with nested valid paths."""
=======
def test_safe_join_subdir(tmp_path: pathlib.Path) -> None:
    """Test safe_join with subdirectory."""
>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
    base = str(tmp_path)
    path = "subdir/file.txt"
    expected = os.path.join(base, "subdir", "file.txt")
    assert safe_join(base, path) == expected


<<<<<<< HEAD
def test_safe_join_traversal(tmp_path: Path) -> None:
    """Test safe_join prevents directory traversal."""
    base = str(tmp_path)
    # Attempt to go up one level
    path = "../secret.txt"
=======
def test_safe_join_traversal(tmp_path: pathlib.Path) -> None:
    """Test safe_join with path traversal."""
    base = str(tmp_path)
    path = "../outside.txt"
>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


<<<<<<< HEAD
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
=======
def test_safe_join_absolute_traversal(tmp_path: pathlib.Path) -> None:
    """Test safe_join with absolute path outside base."""
    base = str(tmp_path)
    # Use a path that is definitely outside tmp_path
    path = "/etc/passwd"
    # But wait, on some systems /tmp might be symlinked.
    # Let's use a path relative to root but different tree

>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)


<<<<<<< HEAD
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

=======
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
>>>>>>> origin/sentinel/fix-path-traversal-14250803625781569863
    with pytest.raises(ValueError, match="Path traversal attempt"):
        safe_join(base, path)
