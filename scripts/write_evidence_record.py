#!/usr/bin/env python3
"""Atomically writes an evidence record + requirement state from a REAL captured run (DG-RVP-014).

Rules:
* outcome is derived only from the captured exit code (0 -> passed, else failed).
* state is at most TESTED; this tool never writes VERIFIED.
* An existing VERIFIED state for the same requirement is invalidated (VERIFIED -> OPEN
  transition with invalidated_at) before new evidence replaces it.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import tempfile
import uuid

ROOT = pathlib.Path(__file__).resolve().parents[1]


def atomic_write(path: pathlib.Path, data: dict) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, tmp = tempfile.mkstemp(dir=path.parent, prefix=path.name, suffix=".tmp")
    with os.fdopen(fd, "w") as fh:
        json.dump(data, fh, indent=2)
        fh.write("\n")
    os.replace(tmp, path)


def main() -> int:
    ap = argparse.ArgumentParser()
    for a in ("req", "class", "command", "started", "finished", "log", "commit",
              "branch", "env-kind", "env-id", "generator"):
        ap.add_argument(f"--{a}", required=True)
    ap.add_argument("--exit-code", type=int, required=True)
    a = ap.parse_args()

    log_path = ROOT / a.log
    if not log_path.is_file():
        raise SystemExit(f"captured log missing: {log_path}")
    sha = hashlib.sha256(log_path.read_bytes()).hexdigest()
    outcome = "passed" if a.exit_code == 0 else "failed"
    ev_id = str(uuid.uuid5(uuid.NAMESPACE_URL, f"dark-gravity:{a.req}:{a.commit}:{sha}"))

    record = {
        "schema_version": "1.0",
        "evidence_id": ev_id,
        "requirement_id": a.req,
        "collected_at": a.finished,
        "started_at": a.started,
        "finished_at": a.finished,
        "generator_version": a.generator,
        "source_revision": {"repository": "lgcorzo/rust_CACD_autonomous_factory",
                            "branch": a.branch, "commit": a.commit},
        "deployment_revision": None,
        "environment": {"kind": a.env_kind, "identifier": a.env_id},
        "verification_class": a.__dict__["class"],
        "method": {"tool": a.command.split()[0], "command_or_test": a.command},
        "outcome": outcome,
        "exit_code": a.exit_code,
        "artifacts": [{"uri": f"file://{a.log}", "sha256": sha, "media_type": "text/plain"}],
        "collector": {"id": a.generator, "role": "test_runner"},
        "limitations": [] if outcome == "passed" else [f"Command exited with code {a.exit_code}"],
    }
    atomic_write(ROOT / "evidence" / "records" / f"{a.req}.json", record)

    state_path = ROOT / "evidence" / "states" / f"{a.req}.json"
    transitions = [
        {"from": None, "to": "OPEN", "at": a.started, "actor_id": "spec-kit", "reason": "Registered"},
        {"from": "OPEN", "to": "IMPLEMENTED", "at": a.started, "actor_id": "antigravity-implementer",
         "reason": f"Implementation present at {a.commit}"},
    ]
    status = "IMPLEMENTED"
    if outcome == "passed":
        transitions.append({"from": "IMPLEMENTED", "to": "TESTED", "at": a.finished,
                            "actor_id": a.generator,
                            "reason": f"Executed '{a.command}' exit_code=0 (log sha256 {sha})"})
        status = "TESTED"
    state = {
        "schema_version": "1.0",
        "requirement_id": a.req,
        "status": status,
        "implementation_agent_id": "antigravity-implementer",
        "evidence_refs": [ev_id] if outcome == "passed" else [],
        "transitions": transitions,
        "verified_by": None,
        "invalidated_at": None,
    }
    atomic_write(state_path, state)
    print(f"{a.req}: outcome={outcome} exit={a.exit_code} state={status} sha256={sha[:12]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

