"""
tests/test_verify_spec_artifacts.py - Negative Tests for Spec Artifacts Governance Gate
"""

import tempfile
import shutil
from pathlib import Path
import pytest
from scripts.verify_spec_artifacts import validate_spec_dir

def test_passes_when_all_three_artifacts_exist_and_coherent():
    with tempfile.TemporaryDirectory() as tmp:
        spec_dir = Path(tmp) / "specs" / "099-test-feature"
        spec_dir.mkdir(parents=True)
        (spec_dir / "spec.md").write_text("# Spec", encoding="utf-8")
        (spec_dir / "plan.md").write_text("# Plan", encoding="utf-8")
        (spec_dir / "tasks.md").write_text("# Tasks\n- [ ] T001 Do something\n", encoding="utf-8")

        errors = validate_spec_dir(spec_dir)
        assert errors == []

def test_fails_when_plan_is_missing():
    with tempfile.TemporaryDirectory() as tmp:
        spec_dir = Path(tmp) / "specs" / "099-test-feature"
        spec_dir.mkdir(parents=True)
        (spec_dir / "spec.md").write_text("# Spec", encoding="utf-8")
        (spec_dir / "tasks.md").write_text("# Tasks\n- [ ] T001 Do something\n", encoding="utf-8")

        errors = validate_spec_dir(spec_dir)
        assert any("Missing plan.md" in err for err in errors)

def test_fails_when_tasks_is_missing():
    with tempfile.TemporaryDirectory() as tmp:
        spec_dir = Path(tmp) / "specs" / "099-test-feature"
        spec_dir.mkdir(parents=True)
        (spec_dir / "spec.md").write_text("# Spec", encoding="utf-8")
        (spec_dir / "plan.md").write_text("# Plan", encoding="utf-8")

        errors = validate_spec_dir(spec_dir)
        assert any("Missing tasks.md" in err for err in errors)

def test_fails_when_completed_task_references_missing_path():
    with tempfile.TemporaryDirectory() as tmp:
        spec_dir = Path(tmp) / "specs" / "099-test-feature"
        spec_dir.mkdir(parents=True)
        (spec_dir / "spec.md").write_text("# Spec", encoding="utf-8")
        (spec_dir / "plan.md").write_text("# Plan", encoding="utf-8")
        (spec_dir / "tasks.md").write_text("- [x] T001 Complete evidence/records/FR-999.json\n", encoding="utf-8")

        errors = validate_spec_dir(spec_dir)
        assert any("references non-existent path" in err for err in errors)

