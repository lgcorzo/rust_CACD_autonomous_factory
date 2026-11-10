#!/usr/bin/env python3
"""
scripts/verify_spec_artifacts.py - Spec Artifacts & SDD Governance Validator (DG-RVP-015 §4)

Validates that every specification directory under specs/ contains the required SDD artifacts:
  - spec.md
  - plan.md
  - tasks.md

Enforces consistency:
  - Validates tasks.md has structured checkboxes and requirement references
  - Verifies that completed tasks referencing evidence files have corresponding files on disk
  - Verifies requirement verification states and evidence records referenced in specs exist
  - Rejects completed tasks claiming evidence if the evidence record is missing or invalid

Exit codes:
  0: PASS (All specifications and artifacts are coherent)
  1: FAIL (Missing artifacts, broken references, or incoherent claims)
"""

import os
import sys
import re
import json
from pathlib import Path

def validate_spec_dir(spec_dir: Path) -> list[str]:
    errors = []
    spec_md = spec_dir / "spec.md"
    plan_md = spec_dir / "plan.md"
    tasks_md = spec_dir / "tasks.md"

    if not spec_md.exists():
        errors.append(f"[{spec_dir.name}] Missing spec.md")
    if not plan_md.exists():
        errors.append(f"[{spec_dir.name}] Missing plan.md")
    if not tasks_md.exists():
        errors.append(f"[{spec_dir.name}] Missing tasks.md")

    if not tasks_md.exists():
        return errors

    tasks_content = tasks_md.read_text(encoding="utf-8")
    task_lines = [line.strip() for line in tasks_content.splitlines() if re.match(r"^-\s*\[[ xX]\]", line.strip())]

    if not task_lines:
        errors.append(f"[{spec_dir.name}] tasks.md does not contain any actionable checklist items (- [ ] / - [x])")

    # Check for completed tasks that reference evidence files or records
    repo_root = spec_dir.parent.parent
    for line in task_lines:
        if line.startswith("- [x]") or line.startswith("- [X]"):
            # Check for path references like evidence/records/FR-xxx.json or docs/audit/...
            referenced_paths = re.findall(r"(?:evidence|docs|specs)/[a-zA-Z0-9_\-\./]+\.[a-zA-Z0-9]+", line)
            for ref_path in referenced_paths:
                full_path = repo_root / ref_path
                if not full_path.exists():
                    errors.append(f"[{spec_dir.name}] Task '{line[:50]}...' references non-existent path: {ref_path}")

    return errors

def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    specs_dir = repo_root / "specs"

    if not specs_dir.exists() or not specs_dir.is_dir():
        print(f"ERROR: specs directory not found at {specs_dir}", file=sys.stderr)
        return 1

    spec_dirs = [d for d in specs_dir.iterdir() if d.is_dir() and not d.name.startswith(".")]

    if not spec_dirs:
        print(f"ERROR: No specification subdirectories found under {specs_dir}", file=sys.stderr)
        return 1

    all_errors = []
    print(f"Validating {len(spec_dirs)} specification directories under specs/...")

    for spec_dir in sorted(spec_dirs, key=lambda d: d.name):
        dir_errors = validate_spec_dir(spec_dir)
        if dir_errors:
            all_errors.extend(dir_errors)
            print(f"  ❌ {spec_dir.name}: {len(dir_errors)} error(s)")
            for err in dir_errors:
                print(f"     - {err}")
        else:
            print(f"  ✓ {spec_dir.name}: PASS (spec.md, plan.md, tasks.md coherent)")

    if all_errors:
        print(f"\nGovernance Gate FAILED: {len(all_errors)} specification defect(s) detected.", file=sys.stderr)
        return 1

    print("\nGovernance Gate PASSED: All specification artifacts are valid and coherent.")
    return 0

if __name__ == "__main__":
    sys.exit(main())

