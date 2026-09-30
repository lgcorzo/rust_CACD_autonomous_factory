# Quickstart Validation Guide: Professional Enterprise Wiki

**Feature Branch**: `007-professional-wiki-docs`  
**Date**: 2026-09-30  
**Spec**: [spec.md](spec.md)  
**Plan**: [plan.md](plan.md)  

---

## Prerequisites

- Git clone of `rust_CACD_autonomous_factory` on branch `007-professional-wiki-docs`
- Markdown viewer supporting Mermaid.js (GitHub.com, GitLab, VS Code with Mermaid extension)
- `grep`, `find`, `wc` (standard Unix tooling for validation)

---

## Validation Scenario 1: Wiki Structure Completeness

**Goal**: Verify all canonical wiki documents exist per [research.md](research.md) Decision 2.

```bash
# From repository root
EXPECTED_FILES=(
  "wiki/Home.md"
  "wiki/_Sidebar.md"
  "wiki/README.md"
  "wiki/index.md"
  "wiki/GLOSSARY.md"
  "wiki/BUSINESS-CONTEXT.md"
  "wiki/STRATEGIC-DESIGN.md"
  "wiki/TACTICAL-DESIGN.md"
  "wiki/AGENT-SPECIFICATIONS.md"
  "wiki/EXPERIMENT-LIFECYCLE.md"
  "wiki/INFRASTRUCTURE-ADAPTERS.md"
  "wiki/VERIFICATION-TRIAD.md"
  "wiki/PRODUCTION-OPERATIONS.md"
  "wiki/USER-MANUAL.md"
)

for f in "${EXPECTED_FILES[@]}"; do
  [ -f "$f" ] && echo "✅ $f" || echo "❌ MISSING: $f"
done
```

**Expected Outcome**: All 14 core documents exist and are non-empty.

---

## Validation Scenario 2: C4 Diagram Coverage

**Goal**: Verify C4 diagrams exist at Context (C1), Container (C2), and Component (C3) levels.

```bash
# Count Mermaid diagram blocks across wiki
echo "=== Mermaid diagram count ==="
grep -r '```mermaid' wiki/ | wc -l

# Verify C4 levels are represented
echo "=== C4 Level references ==="
grep -ril "C1.*Context\|System Context\|Context Diagram" wiki/
grep -ril "C2.*Container\|Container Diagram" wiki/
grep -ril "C3.*Component\|Component Diagram" wiki/
```

**Expected Outcome**: ≥ 3 distinct C4 levels covered; ≥ 15 total Mermaid diagram blocks.

---

## Validation Scenario 3: Agent Specification Coverage

**Goal**: Verify all 6 factory agents are documented with behavioral specs.

```bash
AGENTS=("Rustant" "ZeroClaw" "Auditor" "FinOps" "QAObserver" "DocAgent")

for agent in "${AGENTS[@]}"; do
  count=$(grep -ril "$agent" wiki/ | wc -l)
  [ "$count" -gt 0 ] && echo "✅ $agent: $count references" || echo "❌ MISSING: $agent"
done
```

**Expected Outcome**: Each agent appears in ≥ 1 wiki file; `AGENT-SPECIFICATIONS.md` contains all 6.

---

## Validation Scenario 4: Internal Link Integrity

**Goal**: Verify zero broken internal wiki cross-references.

```bash
# Extract all markdown links and check targets exist
grep -roP '\[.*?\]\(((?!http)[^)]+\.md)\)' wiki/ | \
  sed 's/.*(\(.*\))/\1/' | \
  while read link; do
    target="wiki/$link"
    [ -f "$target" ] && echo "✅ $target" || echo "❌ BROKEN: $target"
  done
```

**Expected Outcome**: Zero broken links reported.

---

## Validation Scenario 5: Mermaid Syntax Validation

**Goal**: Verify all Mermaid blocks use valid syntax (no HTML tags in labels, quoted special characters).

```bash
# Check for common Mermaid syntax errors
echo "=== HTML tags in Mermaid blocks (should be 0) ==="
awk '/```mermaid/,/```/' wiki/*.md | grep -c '<[a-z]' || echo "0 found"

echo "=== Unquoted parentheses in node labels (should be 0) ==="
awk '/```mermaid/,/```/' wiki/*.md | grep -P '\w+\[.*\(.*\).*\]' | head -5
```

**Expected Outcome**: Zero HTML tags and zero unquoted parentheses in Mermaid blocks.

---

## Validation Scenario 6: Per-Crate OKF Source Map Coverage

**Goal**: Verify every crate has corresponding OKF source map files.

```bash
CRATES=("factory-core" "factory-application" "factory-infrastructure" "factory-mcp-server" "factory-cli")

for crate in "${CRATES[@]}"; do
  count=$(find wiki/ -name "crates_${crate}*" | wc -l)
  echo "$crate: $count OKF source maps"
done
```

**Expected Outcome**: Each crate has ≥ 1 OKF source map; `factory-application` and `factory-mcp-server` have ≥ 10 each.

---

## Validation Scenario 7: Security & Compliance Documentation

**Goal**: Verify Zero Trust security and R&D compliance sections exist.

```bash
echo "=== Security coverage ==="
grep -ril "Ed25519\|Non-Human Identity\|NHI" wiki/ | wc -l
grep -ril "OpenZiti\|Zero Trust" wiki/ | wc -l
grep -ril "gVisor\|sandbox.*isolation" wiki/ | wc -l

echo "=== Compliance coverage ==="
grep -ril "Hazitek\|SPRI\|EU AI Act" wiki/ | wc -l
grep -ril "FinOps\|compute.*hours\|token.*consumption" wiki/ | wc -l
```

**Expected Outcome**: ≥ 3 files reference each security mechanism; ≥ 2 files reference compliance frameworks.

---

## OSR < 5% Quality Gate (SC-007)

**Goal**: Verify Orphan Symbol Rate stays below 5%.

```bash
# Extract public Rust symbols
cargo doc --workspace --no-deps 2>/dev/null
# Compare documented symbols against wiki references
# OSR = (undocumented_symbols / total_public_symbols) * 100
# Target: OSR < 5%
```

> **Note**: Full OSR calculation requires the `deepwiki-rs` AST parser or manual extraction with `tree-sitter`. See [data-model.md](data-model.md) for the `WikiDocument.ast_symbols_covered` field definition.
