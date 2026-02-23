"""Tests for security utilities."""

import os
import pytest
from autogen_team.core.security import safe_join


def test_safe_join_valid_path(tmp_path: str) -> None:
    """Test safe_join with a valid relative path."""
    base = str(tmp_path)
    result = safe_join(base, "foo", "bar.txt")
    expected = os.path.join(base, "foo", "bar.txt")
    assert result == expected


def test_safe_join_path_traversal(tmp_path: str) -> None:
    """Test safe_join detects path traversal attempts."""
    base = str(tmp_path)
    with pytest.raises(ValueError, match="Path traversal detected"):
        safe_join(base, "..", "secret.txt")


def test_safe_join_absolute_path_escape(tmp_path: str) -> None:
    """Test safe_join detects absolute path escape."""
    base = str(tmp_path)
    with pytest.raises(ValueError, match="Path traversal detected"):
        safe_join(base, "/etc/passwd")


def test_safe_join_nested_traversal(tmp_path: str) -> None:
    """Test safe_join with nested traversal that resolves outside."""
    base = str(tmp_path)
    # base/foo/../../bar resolves to base/../bar which is outside base
    with pytest.raises(ValueError, match="Path traversal detected"):
        safe_join(base, "foo", "../../bar")


def test_safe_join_valid_nested_traversal(tmp_path: str) -> None:
    """Test safe_join with nested traversal that stays inside."""
    base = str(tmp_path)
    # base/foo/../bar resolves to base/bar which is inside base
    result = safe_join(base, "foo", "../bar")
    expected = os.path.join(base, "bar")
    assert result == expected
