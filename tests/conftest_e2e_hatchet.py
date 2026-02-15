# Minimal conftest for Hatchet E2E
import pytest


@pytest.fixture(scope="session")
def tests_path() -> str:
    return "tests"
