# Public Engineering Site Review: `lgcorzo.github.io` (`FR-024`, `FR-025`, `FR-029`, `SC-008`, `SC-011`)

**Repository**: `https://github.com/lgcorzo/lgcorzo.github.io.git` (`/mnt/F024B17C24B145FE/Repos/lgcorzo.github.io`)  
**Base Revision**: `main` @ `e69db33ccceb3e7fdfaaa7bca5d593dedb296f7c`  
**Evaluated At (UTC)**: `2026-11-06T18:12:07Z`  

---

## 1. Canonical Claims & Contradiction Remediation (`T038`–`T042`)

A dated canonical claims registry (`data/canonical-claims.json`), deterministic validator (`scripts/check_public_claims.py`), pytest regression suite (`tests/test_public_claims.py`), and unprivileged GitHub Actions workflow (`.github/workflows/site-consistency.yml`) were implemented and verified:

| Finding / Claim Area | Pre-Remediation State (`main@e69db33c`) | Remediation Applied | Verification Method | Status |
|---|---|---|---|---|
| **Repository Statistics (`64` vs `69`)** | `index.html` and `fork-policy.html` stated `64 public repositories (22 original + 42 forks)`, while `notebooks.html` (`EXP-2026-011`) stated `69 repositories total` without explaining the 5 private/archived repositories. | Clarified in `notebooks.html` and `data/canonical-claims.json`: `69 repositories total (64 public: 22 original + 42 forks, plus 5 private/archived)`. | `test_no_contradictory_unqualified_69_repositories_claim` | `PASS` |
| **Mission Workflow Phases (`5` vs `6`)** | `index.html`, `architecture.html`, and `lifecycle.html` stated `5 Hatchet steps`, while `notebooks.html` (`#screencast-demo`) stated `6-phase DAG`. | Clarified in `notebooks.html` that the 6-stage screencast flow consists of `Kafka ingestion + 5 Hatchet DAG phases`. | `test_hatchet_phase_count_consistency_across_pages` | `PASS` |
| **EU AI Act & SOC 2 Qualification** | `index.html` stated `No third-party certification (EU AI Act, SOC 2 or other) is claimed`, while `notebooks.html` line 351 stated `stored for EU AI Act Conformity Assessment & SOC 2 Type II compliance`. | Reconciled `notebooks.html` line 351 to state `architectural readiness (no third-party certification claimed)`. | `test_no_unqualified_eu_ai_act_or_soc2_compliance_claim` | `PASS` |
| **Anchor Target on `lifecycle.html`** | `lifecycle.html` line 151 linked to `notebooks.html#exp-2026-011` (Fleet Remediation) instead of `notebooks.html#exp-2026-011-adr008` for the `Lab note on ADR-008`. | Updated link target in `lifecycle.html` to `notebooks.html#exp-2026-011-adr008`. | Static HTML link & fragment parser check (`0` broken internal links/anchors) | `PASS` |

---

## 2. Measured Site Quality, Accessibility, Responsive Design, and Performance (`T043`)

Per `FR-029` and `SC-011`, only directly measured attributes are marked `MEASURED / VERIFIED`; browser-rendered Lighthouse/CWV metrics that were not run in a headless Chromium instance are explicitly marked `UNMEASURED`:

| Page | Payload Size (KiB) | `<html lang="en">` & Viewport Meta | `<title>`, `<meta description>`, Canonical & OG Tags | Skip Link (`.skip-link`) & `<main id="main">` | Broken Internal Links / Anchors |
|---|---:|:---:|:---:|:---:|:---:|
| `index.html` | `24.0 KiB` | `PASS` | `PASS` | `PASS` | `0` |
| `architecture.html` | `19.9 KiB` | `PASS` | `PASS` | `PASS` | `0` |
| `lifecycle.html` | `13.3 KiB` | `PASS` | `PASS` | `PASS` | `0` |
| `notebooks.html` | `67.1 KiB` | `PASS` | `PASS` | `PASS` | `0` |
| `fork-policy.html` | `24.4 KiB` | `PASS` | `PASS` | `PASS` | `0` |
| `sovereignty.html` (redirect) | `1.1 KiB` | `PASS` | `PASS` (`http-equiv="refresh"` to `fork-policy.html`) | `N/A` (Redirect stub) | `0` |

### 2.1 Responsive & Accessibility CSS Inspection (`assets/site.css` — `20,349 bytes`)
- **Measured Media Queries**:
  - `@media (prefers-color-scheme: light)`: Light/dark theme support.
  - `@media (prefers-reduced-motion: reduce)`: Disables smooth scroll/transitions for motion-sensitive users.
  - `@media (max-width: 768px)` and `@media (max-width: 560px)`: Responsive single-column collapse and horizontal table scrolling (`.table-container`).
  - `@media print`: Print-friendly layout rules.
- **Semantic Structure**:
  - All 5 primary content pages use `<header class="site-header">`, `<nav class="site-nav" aria-label="Main">`, `aria-current="page"`, `<nav class="toc" aria-label="On this page">`, `<main id="main">`, `<th scope="col">`, `<abbr title="...">`, and `<footer class="site-footer">`.
- **Unmeasured Attributes (`FR-029`, `SC-011`)**:
  - Live browser Core Web Vitals (LCP, CLS, INP) and automated WCAG AA color-contrast ratio calculation across rendered DOM states were not executed via a headless browser during this run and are therefore classified as **`UNMEASURED` (not claimed as verified)**. Static payload sizes (`13.3 KiB`–`67.1 KiB` HTML + `19.9 KiB` CSS, zero external JS bundles) are `MEASURED`.

