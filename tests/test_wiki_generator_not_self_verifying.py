"""DG-RVP-018 A2: generated wiki pages must never claim verified: "true"."""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GENERATORS = [ROOT / "scripts" / "generate_openwiki.py", ROOT / ".agents" / "tools" / "okf_generator.py"]


def test_generator_templates_do_not_stamp_verified_true():
    for gen in GENERATORS:
        text = gen.read_text(encoding="utf-8")
        assert not re.search(r'verified:\s*"?true"?', text), f"{gen} stamps verified: true"


def test_openwiki_template_records_generation_commit_and_unverified():
    text = (ROOT / "scripts" / "generate_openwiki.py").read_text(encoding="utf-8")
    assert 'verified: "false"' in text
    assert "generated_from_commit:" in text


def test_committed_generated_pages_have_no_true_without_reviewer():
    bad = []
    for base in (ROOT / "openwiki", ROOT / "wiki"):
        if not base.is_dir():
            continue
        for md in base.rglob("*.md"):
            head = md.read_text(encoding="utf-8", errors="ignore").split("---", 2)
            if len(head) < 3:
                continue
            fm = head[1]
            if re.search(r'^verified:\s*"?true"?\s*$', fm, re.M) and not re.search(
                r'^reviewer:\s*"?\S+', fm, re.M
            ):
                bad.append(str(md.relative_to(ROOT)))
    assert not bad, f"{len(bad)} pages claim verified: true without reviewer, e.g. {bad[:5]}"
