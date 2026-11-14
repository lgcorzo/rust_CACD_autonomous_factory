"""Wiki documentation quality checks (DG-RVP-018 C3).

Replaces brittle exact-file-count assertions with structural checks:
1. Every wiki Markdown page has valid YAML front matter.
2. No broken internal relative links.
3. No page claims `verified: true` / `verified: "true"` without a recorded reviewer.
"""

from pathlib import Path
import re

WIKI_DIR = Path(__file__).resolve().parent.parent / "wiki"
FRONTMATTER_RE = re.compile(r"^---\s*\n(.*?)\n---\s*\n", re.DOTALL)
MD_LINK_RE = re.compile(r"\[[^\]]+\]\(([^)]+)\)")


def _iter_wiki_pages():
    if not WIKI_DIR.exists():
        return []
    return sorted(
        p for p in WIKI_DIR.rglob("*.md") if not p.name.startswith("_")
    )


def test_wiki_pages_have_valid_frontmatter():
    pages = _iter_wiki_pages()
    assert pages, "Expected at least one wiki page in wiki/"
    for page in pages:
        text = page.read_text(encoding="utf-8")
        match = FRONTMATTER_RE.match(text)
        assert match is not None, f"{page.relative_to(WIKI_DIR)} missing YAML front matter"


def test_wiki_pages_no_unreviewed_verified_true():
    for page in _iter_wiki_pages():
        text = page.read_text(encoding="utf-8")
        match = FRONTMATTER_RE.match(text)
        if not match:
            continue
        fm = match.group(1)
        if re.search(r'^verified:\s*"?true"?\s*$', fm, re.MULTILINE):
            has_reviewer = re.search(
                r"^(verified_by|reviewer):\s*\S+", fm, re.MULTILINE
            )
            assert has_reviewer is not None, (
                f"{page.relative_to(WIKI_DIR)} claims verified: true without verified_by/reviewer"
            )


def test_wiki_pages_no_broken_internal_links():
    for page in _iter_wiki_pages():
        text = page.read_text(encoding="utf-8")
        # Strip fenced code blocks before scanning links
        stripped = re.sub(r"```.*?```", "", text, flags=re.DOTALL)
        for target in MD_LINK_RE.findall(stripped):
            if target.startswith(("http://", "https://", "mailto:", "#")):
                continue
            rel_path = target.split("#", 1)[0].strip()
            if not rel_path:
                continue
            resolved = (page.parent / rel_path).resolve()
            assert resolved.exists(), (
                f"{page.relative_to(WIKI_DIR)} has broken internal link: {target}"
            )
