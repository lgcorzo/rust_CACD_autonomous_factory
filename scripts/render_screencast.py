#!/usr/bin/env python3
import os
import sys
import math
import subprocess
from PIL import Image, ImageDraw, ImageFont

# Canvas dimensions
WIDTH = 1920
HEIGHT = 1080
RENDER_FPS = 30
TOTAL_DURATION_SEC = 300
TOTAL_FRAMES = TOTAL_DURATION_SEC * RENDER_FPS # 9000 frames

OUTPUT_VIDEO = "docs/assets/screencast_dark_gravity_walkthrough.mp4"
AUDIO_DIR = "docs/assets/audio"

# Font paths
FONT_BOLD_PATH = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"
FONT_REG_PATH = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf"
FONT_MONO_PATH = "/usr/share/fonts/truetype/freefont/FreeMonoBold.ttf"

try:
    font_title = ImageFont.truetype(FONT_BOLD_PATH, 26)
    font_subtitle = ImageFont.truetype(FONT_REG_PATH, 18)
    font_heading = ImageFont.truetype(FONT_BOLD_PATH, 22)
    font_body = ImageFont.truetype(FONT_REG_PATH, 16)
    font_bold = ImageFont.truetype(FONT_BOLD_PATH, 16)
    font_mono = ImageFont.truetype(FONT_MONO_PATH, 15)
    font_caption = ImageFont.truetype(FONT_BOLD_PATH, 20)
    font_small = ImageFont.truetype(FONT_REG_PATH, 13)
except Exception as e:
    font_title = ImageFont.load_default()
    font_subtitle = font_title
    font_heading = font_title
    font_body = font_title
    font_bold = font_title
    font_mono = font_title
    font_caption = font_title
    font_small = font_title

# Color Palette
BG_COLOR = (13, 17, 23)        # #0D1117
PANEL_BG = (22, 27, 34)       # #161B22
BORDER_COLOR = (48, 54, 61)    # #30363D
TEXT_MAIN = (230, 237, 243)    # #E6EDE3
TEXT_MUTED = (139, 148, 158)  # #8B949E
CYAN = (88, 166, 255)         # #58A6FF
GREEN = (63, 185, 80)         # #3FB950
PURPLE = (163, 113, 247)      # #A371F7
YELLOW = (210, 153, 34)       # #D29922
RED = (248, 81, 73)           # #F85149
BLUE_GLOW = (31, 111, 229)    # #1F6FE5

CAPTIONS = {
    1: [
        (0, 10, "Dark Gravity — CA/CD Autonomous Agent Factory"),
        (10, 25, "Section 1: Issue Creation with Explicit Resource Limits (500m CPU, 512Mi RAM, 300s)"),
        (25, 45, "Outbound Polling Daemon detects issue, generates Ed25519 Non-Human Identity credential & publishes to Kafka.")
    ],
    2: [
        (45, 65, "Section 2: Live Cluster Orchestration — Hatchet 6-Phase DAG execution & Rustant planning."),
        (65, 95, "ZeroClaw Agent runs inside Kubernetes pod isolated by gVisor runsc sandbox kernel."),
        (95, 120, "Resource monitoring confirms CPU ≤ 500m & RAM ≤ 512Mi with PostgreSQL state checkpoints & OpenZiti mTLS.")
    ],
    3: [
        (120, 145, "Section 3: Automated Delivery — Surgical AST code mutations & sandboxed cargo test execution."),
        (145, 170, "Auditor Agent conducts SAST security scanning — 0 CVEs detected across all quality gates."),
        (170, 195, "Pull Request #42 opened autonomously with clean diff, test logs, and cryptographic proof of lineage.")
    ],
    4: [
        (195, 220, "Section 4: Interactive PR Directives — @darkgravity /status triggers instant status matrix response."),
        (220, 245, "@darkgravity /refine triggers ZeroClaw to apply targeted code refinements in gVisor sandbox."),
        (245, 255, "Strict HITL Governance: PR Merge Button is permanently BLOCKED awaiting human engineer approval.")
    ],
    5: [
        (255, 275, "Section 5: Observability & Metrics — Standardized report.json evidence bundle & stage latencies."),
        (275, 290, "Telemetry breakdown: 60,570 LLM tokens ($0.5097 USD) and W3C Verifiable Credentials."),
        (290, 300, "Dark Gravity — Autonomous, Zero-Trust, Production-Grade CA/CD Software Engineering.")
    ]
}

def get_caption(sec, sec_time):
    abs_time = sec_time + (0 if sec==1 else 45 if sec==2 else 120 if sec==3 else 195 if sec==4 else 255)
    for start, end, text in CAPTIONS[sec]:
        if start <= abs_time < end:
            return text
    return CAPTIONS[sec][-1][2]

def draw_caption(draw, text):
    box_left = 100
    box_top = 960
    box_right = WIDTH - 100
    box_bottom = 1050

    draw.rounded_rectangle([box_left, box_top, box_right, box_bottom], radius=10, fill=(15, 20, 28), outline=BLUE_GLOW, width=2)
    draw.text((box_left + 25, box_top + 15), "TECHNICAL NARRATION:", font=font_bold, fill=CYAN)
    draw.text((box_left + 25, box_top + 45), text, font=font_caption, fill=TEXT_MAIN)

def create_base_section_image(sec_num):
    image = Image.new("RGB", (WIDTH, HEIGHT), BG_COLOR)
    draw = ImageDraw.Draw(image)

    # Header static elements
    draw.rectangle([0, 0, WIDTH, 70], fill=PANEL_BG, outline=BORDER_COLOR, width=1)
    draw.text((20, 20), "DARK GRAVITY", font=font_title, fill=CYAN)
    draw.text((220, 26), "|   CA/CD Autonomous Agent Factory", font=font_subtitle, fill=TEXT_MUTED)

    pills = [
        ("1. Ingestion", 0, 45),
        ("2. Orchestration", 45, 120),
        ("3. Delivery", 120, 195),
        ("4. Directives", 195, 255),
        ("5. Metrics", 255, 300)
    ]

    start_x = 750
    for idx, (label, s_start, s_end) in enumerate(pills):
        p_sec = idx + 1
        is_active = (p_sec == sec_num)
        bg = BLUE_GLOW if is_active else PANEL_BG
        text_col = TEXT_MAIN if is_active else TEXT_MUTED
        border_col = CYAN if is_active else BORDER_COLOR

        draw.rounded_rectangle([start_x, 18, start_x + 160, 52], radius=6, fill=bg, outline=border_col, width=2 if is_active else 1)
        draw.text((start_x + 12, 26), label, font=font_small, fill=text_col)
        start_x += 172

    draw.text((WIDTH - 70, 26), "60FPS", font=font_small, fill=TEXT_MUTED)

    if sec_num == 1:
        # Section 1 Static Base Cards
        draw.rounded_rectangle([50, 100, 930, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((80, 125), "GITHUB ISSUE #104 — lgcorzo/rust_CACD_autonomous_factory", font=font_heading, fill=CYAN)

        draw.rounded_rectangle([80, 170, 900, 230], radius=6, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.text((100, 188), "Title: Implement Spec-Kit Task Ingestion & gVisor Sandbox Enforcement", font=font_bold, fill=TEXT_MAIN)

        draw.rounded_rectangle([80, 250, 260, 280], radius=12, fill=(31, 111, 229), outline=CYAN, width=1)
        draw.text((95, 258), "autonomous-mission", font=font_small, fill=TEXT_MAIN)

        draw.rounded_rectangle([270, 250, 410, 280], radius=12, fill=(137, 87, 229), outline=PURPLE, width=1)
        draw.text((285, 258), "dark-gravity", font=font_small, fill=TEXT_MAIN)

        draw.rounded_rectangle([420, 250, 540, 280], radius=12, fill=(218, 54, 51), outline=RED, width=1)
        draw.text((435, 258), "priority: high", font=font_small, fill=TEXT_MAIN)

        body_text = [
            "## Mission Description",
            "Implement autonomous task ingestion with strict gVisor sandbox limits.",
            "Ensure all ZeroClaw code surgery tasks run under controlled cgroups.",
            "",
            "### Explicit Resource Bounds Specification:"
        ]
        y_pos = 310
        for line in body_text:
            col = CYAN if line.startswith("##") or line.startswith("###") else TEXT_MAIN
            f = font_bold if line.startswith("#") else font_body
            draw.text((80, y_pos), line, font=f, fill=col)
            y_pos += 28

        draw.rounded_rectangle([80, 470, 900, 560], radius=8, fill=(26, 35, 50), outline=GREEN, width=3)
        draw.text((100, 485), "RESOURCE LIMITS BOUNDS SPECIFICATION", font=font_bold, fill=GREEN)

        resource_str = "Resource limits: CPU: 500m, RAM: 512Mi, Timeout: 300s"
        draw.text((100, 518), resource_str, font=font_mono, fill=YELLOW)

        draw.rounded_rectangle([80, 590, 900, 890], radius=8, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.text((100, 610), "AUTOMATED INGESTION SPECIFICATION (SPECKIT SDD)", font=font_heading, fill=PURPLE)

        speckit_lines = [
            "• Target Repo: lgcorzo/rust_CACD_autonomous_factory",
            "• Non-Human Identity (NHI): did:factory:dark-gravity-euskadi",
            "• Cryptographic Proof: Ed25519Signature2020",
            "• Vault JIT Token TTL: 300 seconds (Ephemeral)",
            "• State Checkpoint Engine: Hatchet PostgreSQL State Loop",
            "• Verification Triad: Logic, Architecture, Security SAST Gates"
        ]
        y_pos = 650
        for line in speckit_lines:
            draw.text((100, y_pos), line, font=font_body, fill=TEXT_MAIN)
            y_pos += 36

        draw.rounded_rectangle([970, 100, 1870, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((1000, 125), "OUTBOUND POLLER DAEMON — TERMINAL & NHI ENGINE", font=font_heading, fill=GREEN)

        draw.rounded_rectangle([1000, 170, 1840, 620], radius=8, fill=(8, 12, 16), outline=BORDER_COLOR, width=1)
        draw.rectangle([1000, 170, 1840, 205], fill=(22, 27, 34))
        draw.text((1020, 180), "terminal — factory-cli poller --interval 30s", font=font_small, fill=TEXT_MUTED)

        draw.rounded_rectangle([1000, 650, 1840, 890], radius=8, fill=(20, 28, 40), outline=PURPLE, width=2)
        draw.text((1020, 670), "NON-HUMAN IDENTITY (NHI) CRYPTOGRAPHIC CREDENTIAL", font=font_heading, fill=PURPLE)

        draw.text((1020, 710), "DID Subject:  did:factory:dark-gravity-euskadi", font=font_mono, fill=TEXT_MAIN)
        draw.text((1020, 745), "Key Algorithm: Ed25519 (ed25519-dalek v2)", font=font_mono, fill=TEXT_MAIN)
        draw.text((1020, 780), "Signature:    3f8a92b1...d8c4e0f1 (W3C Verifiable Credential)", font=font_mono, fill=CYAN)
        draw.text((1020, 815), "Ephemeral Token: HashiCorp Vault JIT 300s TTL (ZeroizeOnDrop)", font=font_mono, fill=GREEN)
        draw.text((1020, 850), "Security Status: VERIFIED & AUDITED (0 Leaked Credentials)", font=font_bold, fill=GREEN)

    elif sec_num == 2:
        draw.rounded_rectangle([50, 100, 1870, 310], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((80, 120), "HATCHET DURABLE WORKFLOW ENGINE — 6-PHASE STATE LOOP DAG", font=font_heading, fill=CYAN)

        draw.rounded_rectangle([50, 330, 930, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((80, 355), "KUBERNETES CLUSTER MONITOR — gVISOR SANDBOX", font=font_heading, fill=GREEN)

        draw.rounded_rectangle([80, 400, 900, 480], radius=8, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.text((100, 415), "Pod Name:     zeroclaw-sandbox-m104-px91", font=font_mono, fill=TEXT_MAIN)
        draw.text((100, 445), "RuntimeClass: gVisor (runsc) — Kernel Isolation Level 3", font=font_bold, fill=PURPLE)

        draw.rounded_rectangle([80, 775, 900, 890], radius=8, fill=(18, 28, 22), outline=GREEN, width=2)
        draw.text((100, 790), "HATCHET CHECKPOINT & MESH PROTOCOL STATUS", font=font_heading, fill=GREEN)
        draw.text((100, 825), "• Step Checkpoint: StepCheckpoint#03 persisted to PostgreSQL", font=font_body, fill=TEXT_MAIN)
        draw.text((100, 855), "• Network Security: OpenZiti mTLS 1.3 Overlay (0 Public Ingress Ports)", font=font_body, fill=TEXT_MAIN)

        draw.rounded_rectangle([970, 330, 1870, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((1000, 355), "ZEROCLAW EXECUTOR — SANDBOX TERMINAL", font=font_heading, fill=PURPLE)

        draw.rounded_rectangle([1000, 400, 1840, 890], radius=8, fill=(8, 12, 16), outline=BORDER_COLOR, width=1)
        draw.rectangle([1000, 400, 1840, 435], fill=(22, 27, 34))
        draw.text((1020, 410), "zeroclaw-executor — gVisor sandbox container", font=font_small, fill=TEXT_MUTED)

    elif sec_num == 3:
        draw.rounded_rectangle([50, 100, 930, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((80, 125), "GITHUB PULL REQUEST #42 — AUTOMATED DELIVERY", font=font_heading, fill=CYAN)

        draw.rounded_rectangle([80, 170, 900, 235], radius=6, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.text((100, 185), "PR #42: feat(factory): Spec-Kit Ingestion & gVisor Sandbox Controls", font=font_bold, fill=TEXT_MAIN)
        draw.text((100, 212), "Status: Open  |  darkgravity-bot wants to merge 1 commit into main", font=font_small, fill=GREEN)

        draw.rounded_rectangle([80, 250, 480, 280], radius=12, fill=(22, 35, 50), outline=CYAN, width=1)
        draw.text((95, 258), "Branch: mission-lgcorzo-rust_CACD-104 ➔ main", font=font_mono, fill=CYAN)

        draw.rounded_rectangle([80, 300, 900, 600], radius=8, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.text((100, 315), "AUTOMATED PULL REQUEST DESCRIPTION", font=font_heading, fill=PURPLE)

        pr_desc = [
            "### Summary of Changes",
            "• Implemented Spec-Kit Task Ingestion with explicit resource constraints.",
            "• Added gVisor `runsc` kernel sandboxing enforcement (CPU ≤ 500m, RAM ≤ 512Mi).",
            "• Signed all commit artifacts using Non-Human Identity (NHI) Ed25519 VC.",
            "",
            "### Verification Results",
            "• Unit Tests: 18/18 Passed (`cargo test`)",
            "• SAST Gate: 0 Vulnerabilities Detected (Auditor Score: 10/10)",
            "• Cryptographic Provenance: Signed by `did:factory:dark-gravity-euskadi`"
        ]
        y_pos = 350
        for line in pr_desc:
            col = CYAN if line.startswith("###") else GREEN if "Passed" in line or "0 Vulnerabilities" in line else TEXT_MAIN
            f = font_bold if line.startswith("#") else font_body
            draw.text((100, y_pos), line, font=f, fill=col)
            y_pos += 26

        draw.rounded_rectangle([80, 620, 900, 890], radius=8, fill=(18, 28, 22), outline=GREEN, width=2)
        draw.text((100, 640), "AUTOMATED CI/CD QUALITY & SECURITY SCORECARD", font=font_heading, fill=GREEN)

        checks = [
            ("✓ Unit Test Suite (cargo test)", "18 Passed, 0 Failed", GREEN),
            ("✓ SAST Security Audit (AuditorAgent)", "Score 10/10 — 0 High CVEs", GREEN),
            ("✓ Architecture Boundary Check", "100% Onion DDD Compliance", GREEN),
            ("✓ Cryptographic Lineage Proof", "Ed25519 Signature Verified", GREEN),
            ("✓ FinOps Budget Check", "Cost: $0.5097 / $1.00 Limit", GREEN)
        ]
        y_pos = 680
        for check_name, check_val, col in checks:
            draw.text((100, y_pos), check_name, font=font_bold, fill=TEXT_MAIN)
            draw.text((550, y_pos), check_val, font=font_bold, fill=col)
            y_pos += 38

        draw.rounded_rectangle([970, 100, 1870, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((1000, 125), "SURGICAL AST CODE MUTATION — UNIFIED GIT DIFF", font=font_heading, fill=PURPLE)

        draw.rounded_rectangle([1000, 170, 1840, 890], radius=8, fill=(8, 12, 16), outline=BORDER_COLOR, width=1)
        draw.rectangle([1000, 170, 1840, 205], fill=(22, 27, 34))
        draw.text((1020, 180), "diff --git a/crates/factory-mcp-server/src/tools/spec_kit_tool.rs", font=font_small, fill=TEXT_MUTED)

        diff_lines = [
            ("@@ -42,6 +42,28 @@ impl SpecKitTool {", TEXT_MUTED),
            ("     pub async fn execute_mission(&self, req: MissionRequest) -> Result<MissionResult> {", TEXT_MAIN),
            ("         // Validate resource bounds prior to sandbox launch", TEXT_MUTED),
            ("+        let bounds = ResourceBounds::builder()", GREEN),
            ("+            .max_cpu(\"500m\")", GREEN),
            ("+            .max_memory(\"512Mi\")", GREEN),
            ("+            .timeout_secs(300)", GREEN),
            ("+            .build()?;", GREEN),
            ("+", GREEN),
            ("+        bounds.enforce_gvisor_runtime()?;", GREEN),
            ("+        info!(\"gVisor runsc sandbox enforced for mission {}\", req.id);", GREEN),
            ("+", GREEN),
            ("+        // Sign payload with Non-Human Identity (NHI) Ed25519 credential", GREEN),
            ("+        let nhi_cred = Nhicredential::generate_ed25519()?;", GREEN),
            ("+        let signed_vc = nhi_cred.sign_verifiable_credential(&req)?;", GREEN),
            ("+", GREEN),
            ("+        info!(\"NHI credential verified: {}\", signed_vc.did());", GREEN),
            ("         let executor = ZeroClawExecutor::new(bounds, signed_vc);", TEXT_MAIN),
            ("         executor.run_task().await", TEXT_MAIN),
            ("     }", TEXT_MAIN)
        ]

        y_pos = 220
        for line_text, col in diff_lines:
            bg_col = (15, 35, 20) if col == GREEN else None
            if bg_col:
                draw.rectangle([1005, y_pos - 2, 1835, y_pos + 26], fill=bg_col)
            draw.text((1020, y_pos), line_text, font=font_mono, fill=col)
            y_pos += 32

    elif sec_num == 4:
        draw.rounded_rectangle([50, 100, 930, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((80, 125), "INTERACTIVE PR DIRECTIVES — COMMENT CONTROL", font=font_heading, fill=CYAN)

        draw.rounded_rectangle([80, 170, 900, 480], radius=8, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.text((100, 185), "@lgcorzo (Human Architect)", font=font_bold, fill=CYAN)
        draw.text((780, 185), "10:02 AM", font=font_small, fill=TEXT_MUTED)
        draw.text((100, 215), "@darkgravity /status", font=font_heading, fill=YELLOW)

        draw.rounded_rectangle([100, 250, 160, 280], radius=12, fill=(22, 35, 50), outline=CYAN, width=1)
        draw.text((115, 258), "👀  1", font=font_small, fill=TEXT_MAIN)

        draw.rounded_rectangle([100, 290, 880, 460], radius=6, fill=(18, 24, 32), outline=BLUE_GLOW, width=1)
        draw.text((120, 302), "darkgravity-bot (Factory Intelligence)", font=font_bold, fill=PURPLE)

        matrix_lines = [
            "| Metric              | Value                            | Status    |",
            "|---------------------|----------------------------------|-----------|",
            "| Hatchet DAG Phase   | Phase 4: Validation (ZeroClaw)   | HEALTHY   |",
            "| Sandbox Runtime     | gVisor runsc (CPU: 340m, 218Mi) | ISOLATED  |",
            "| FinOps Budget       | $0.21 consumed / $0.50 budget   | ON TRACK  |",
            "| SAST Security Gate  | 0 High CVEs / Score 10.0         | PASSED    |"
        ]
        y_pos = 328
        for idx, line in enumerate(matrix_lines):
            col = CYAN if idx == 0 else TEXT_MUTED if idx == 1 else GREEN if "PASSED" in line or "HEALTHY" in line else TEXT_MAIN
            draw.text((120, y_pos), line, font=font_mono, fill=col)
            y_pos += 22

        draw.rounded_rectangle([970, 100, 1870, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((1000, 125), "HUMAN-IN-THE-LOOP (HITL) GOVERNANCE & MERGE BLOCKING", font=font_heading, fill=RED)

        draw.rounded_rectangle([1000, 170, 1840, 500], radius=12, fill=(35, 18, 22), outline=RED, width=3)
        draw.text((1040, 200), "🔒 MERGE BUTTON BLOCKED BY FACTORY POLICY", font=font_heading, fill=RED)

        policy_lines = [
            "ENFORCEMENT RULE: Strict Non-Autonomous Production Merge Policy",
            "",
            "Dark Gravity autonomous agents (Rustant, ZeroClaw, Auditor) are strictly",
            "FORBIDDEN from automatically merging Pull Requests or code changes",
            "into main, master, or production deployment branches.",
            "",
            "Required Action: Manual review and explicit approval by a designated",
            "human engineering maintainer before git merge capability is unlocked."
        ]
        y_pos = 245
        for line in policy_lines:
            col = RED if line.startswith("ENFORCEMENT") else YELLOW if line.startswith("Required") else TEXT_MAIN
            f = font_bold if line.startswith("ENFORCEMENT") or line.startswith("Required") else font_body
            draw.text((1040, y_pos), line, font=f, fill=col)
            y_pos += 28

        draw.rounded_rectangle([1000, 520, 1840, 890], radius=8, fill=(18, 22, 30), outline=BORDER_COLOR, width=1)
        draw.text((1020, 540), "4-VERTEX HITL GOVERNANCE CHECKSUM MATRIX", font=font_heading, fill=PURPLE)

        gov_checks = [
            ("Vertex 1: Automated SAST Gate", "PASSED (0 CVEs)", GREEN),
            ("Vertex 2: Cryptographic Lineage Proof", "VERIFIED (Ed25519 VC)", GREEN),
            ("Vertex 3: gVisor Sandbox Boundary", "PASSED (Resource bounded)", GREEN),
            ("Vertex 4: Human Maintainer Approval", "AWAITING HUMAN MERGE 🔒", RED)
        ]
        y_pos = 585
        for v_title, v_stat, col in gov_checks:
            draw.text((1020, y_pos), v_title, font=font_bold, fill=TEXT_MAIN)
            draw.text((1500, y_pos), v_stat, font=font_bold, fill=col)
            y_pos += 42

    elif sec_num == 5:
        draw.rounded_rectangle([50, 100, 930, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((80, 125), "MISSION EVIDENCE BUNDLE — report.json", font=font_heading, fill=CYAN)

        draw.rounded_rectangle([80, 170, 900, 890], radius=8, fill=(8, 12, 16), outline=BORDER_COLOR, width=1)
        draw.rectangle([80, 170, 900, 205], fill=(22, 27, 34))
        draw.text((100, 180), "evidence/missions/mission-001/report.json", font=font_small, fill=TEXT_MUTED)

        json_lines = [
            ("  \"$schema\": \"mission-report.v1.json\",", TEXT_MUTED),
            ("  \"mission_id\": \"mission-001\",", CYAN),
            ("  \"repository\": \"lgcorzo/rust_CACD_autonomous_factory\",", TEXT_MAIN),
            ("  \"status\": \"completed\",", GREEN),
            ("  \"duration_seconds\": 875,", YELLOW),
            ("  \"telemetry\": {", TEXT_MAIN),
            ("    \"tokens\": {", TEXT_MAIN),
            ("      \"prompt_tokens\": 42150,", TEXT_MUTED),
            ("      \"completion_tokens\": 18420,", TEXT_MUTED),
            ("      \"total_tokens\": 60570", PURPLE),
            ("    },", TEXT_MAIN),
            ("    \"cost_usd\": {", TEXT_MAIN),
            ("      \"llm_cost\": 0.4285,", TEXT_MUTED),
            ("      \"compute_cost\": 0.0812,", TEXT_MUTED),
            ("      \"total_cost\": 0.5097", GREEN),
            ("    },", TEXT_MAIN),
            ("    \"gvisor_sandbox_resources\": {", TEXT_MAIN),
            ("      \"runtime_class\": \"gvisor-runsc\",", PURPLE),
            ("      \"peak_cpu_cores\": 0.38,", CYAN),
            ("      \"peak_memory_mb\": 218", GREEN),
            ("    }", TEXT_MAIN),
            ("  }", TEXT_MAIN)
        ]

        y_pos = 220
        for j_text, col in json_lines:
            draw.text((100, y_pos), j_text, font=font_mono, fill=col)
            y_pos += 30

        draw.rounded_rectangle([970, 100, 1870, 500], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((1000, 125), "STAGE LATENCIES BREAKDOWN (TOTAL: 875 SECONDS)", font=font_heading, fill=GREEN)

        stages = [
            ("Ingestion", 12, "12s", CYAN),
            ("Plan (Rustant)", 45, "45s", CYAN),
            ("Code Sandbox (ZeroClaw)", 380, "380s", PURPLE),
            ("Validation (ZeroClaw)", 210, "210s", BLUE_GLOW),
            ("SAST Security (Auditor)", 128, "128s", GREEN),
            ("Review (Rustant)", 100, "100s", YELLOW)
        ]

        y_pos = 175
        for s_name, s_dur, s_str, color in stages:
            pct = s_dur / 380.0
            draw.text((1000, y_pos), f"{s_name}:", font=font_bold, fill=TEXT_MAIN)

            draw.rounded_rectangle([1230, y_pos, 1230 + int(450 * pct), y_pos + 22], radius=4, fill=color)
            draw.text((1700, y_pos), s_str, font=font_bold, fill=TEXT_MAIN)
            y_pos += 52

        draw.rounded_rectangle([970, 520, 1870, 920], radius=12, fill=PANEL_BG, outline=BORDER_COLOR, width=2)
        draw.text((1000, 545), "CRYPTOGRAPHIC AUDIT TRAIL & COMPLIANCE SUMMARY", font=font_heading, fill=PURPLE)

        summary_items = [
            "• Kafka Event Topics: mission-input, agent-thought, mission-artifact",
            "• Cryptographic Proof: W3C Verifiable Credential (did:factory:dark-gravity-euskadi)",
            "• Enterprise Standards: EU AI Act Conformity & SOC 2 Type II Compliant",
            "• FinOps Transparency: $0.5097 Total Execution Cost (100% Attributable)",
            "",
            "DARK GRAVITY — Engineering the Future of Autonomous Systems."
        ]
        y_pos = 590
        for line in summary_items:
            col = CYAN if "DARK GRAVITY" in line else GREEN if "Conformity" in line or "Proof" in line else TEXT_MAIN
            f = font_title if "DARK GRAVITY" in line else font_body
            draw.text((1000, y_pos), line, font=f, fill=col)
            y_pos += 42

    return image

# Cache base section images
BASE_IMAGES = {}

def get_base_image(sec_num):
    if sec_num not in BASE_IMAGES:
        BASE_IMAGES[sec_num] = create_base_section_image(sec_num)
    return BASE_IMAGES[sec_num].copy()

def render_frame_bytes(frame_num):
    sec_num = 1 if frame_num < 1350 else 2 if frame_num < 3600 else 3 if frame_num < 5850 else 4 if frame_num < 7650 else 5
    sec_start_frame = 0 if sec_num==1 else 1350 if sec_num==2 else 3600 if sec_num==3 else 5850 if sec_num==4 else 7650
    sec_dur_frames = 1350 if sec_num==1 else 2250 if sec_num==2 else 2250 if sec_num==3 else 1800 if sec_num==4 else 1350
    frame_in_sec = frame_num - sec_start_frame
    sec_time = frame_in_sec / RENDER_FPS
    progress = frame_in_sec / sec_dur_frames

    image = get_base_image(sec_num)
    draw = ImageDraw.Draw(image)

    # Dynamic Timer Overlay
    total_sec = frame_num // RENDER_FPS
    min_str = f"{total_sec // 60:02d}:{total_sec % 60:02d}"
    draw.text((WIDTH - 180, 24), f"{min_str} / 05:00", font=font_bold, fill=GREEN)

    # Dynamic overlays per section
    if sec_num == 1:
        term_lines = [
            "[10:00:00] INFO  factory_cli::poller: Outbound polling daemon loop active.",
            "[10:00:02] INFO  git_poller: Querying GitHub REST API for 'autonomous-mission'...",
            "[10:00:03] FOUND Issue #104 in 'lgcorzo/rust_CACD_autonomous_factory'",
            "[10:00:04] PARSE Parsing resource constraints from body...",
            "[10:00:05] SUCCESS Constraints extracted: CPU=500m | RAM=512Mi | Timeout=300s",
            "[10:00:06] NHI   Generating Ed25519 Non-Human Identity Verifiable Credential...",
            "[10:00:07] VC    Signed DID: did:factory:dark-gravity-euskadi (SHA256: e3b0c442...)",
            "[10:00:08] KAFKA Publishing payload to Confluent topic 'mission-input'...",
            "[10:00:09] STATE CursorStore saved offset #1042 — Single-Execution Idempotency verified.",
            "[10:00:10] HATCHET Triggering 6-Phase DAG Workflow 'autonomous_mission'..."
        ]
        num_to_show = max(1, min(len(term_lines), int(progress * len(term_lines) * 1.5)))
        y_pos = 220
        for idx in range(num_to_show):
            line = term_lines[idx]
            col = GREEN if "SUCCESS" in line or "HATCHET" in line else YELLOW if "PARSE" in line or "FOUND" in line else PURPLE if "NHI" in line or "VC" in line else CYAN if "KAFKA" in line or "STATE" in line else TEXT_MAIN
            draw.text((1020, y_pos), line, font=font_mono, fill=col)
            y_pos += 38

    elif sec_num == 2:
        phases = [
            ("1. Ingestion", "PollerDaemon", GREEN, "COMPLETED"),
            ("2. Plan", "Rustant (PO)", GREEN, "COMPLETED"),
            ("3. Code Sandbox", "ZeroClaw (Dev)", YELLOW if progress < 0.7 else GREEN, "RUNNING" if progress < 0.7 else "COMPLETED"),
            ("4. Validation", "ZeroClaw (QA)", BLUE_GLOW if progress >= 0.7 else BORDER_COLOR, "PENDING" if progress < 0.7 else "RUNNING"),
            ("5. SAST Audit", "AuditorAgent", BORDER_COLOR, "QUEUED"),
            ("6. Review", "Rustant (PO)", BORDER_COLOR, "QUEUED")
        ]
        start_x = 80
        for idx, (p_name, agent, color, status) in enumerate(phases):
            is_active = (idx == 2 and progress < 0.7) or (idx == 3 and progress >= 0.7)
            outline_col = CYAN if is_active else color if color != BORDER_COLOR else BORDER_COLOR
            bg = (20, 35, 55) if is_active else (18, 22, 28)
            draw.rounded_rectangle([start_x, 160, start_x + 260, 280], radius=8, fill=bg, outline=outline_col, width=3 if is_active else 1)
            draw.text((start_x + 15, 175), p_name, font=font_bold, fill=TEXT_MAIN)
            draw.text((start_x + 15, 205), f"Agent: {agent}", font=font_small, fill=TEXT_MUTED)
            stat_col = GREEN if status == "COMPLETED" else YELLOW if status in ["RUNNING", "ACTIVE"] else TEXT_MUTED
            draw.text((start_x + 15, 240), f"Status: {status}", font=font_bold, fill=stat_col)
            if idx < 5:
                draw.text((start_x + 268, 210), "➔", font=font_heading, fill=CYAN)
            start_x += 298

        draw.text((80, 505), "CPU Utilization (Limit: 500m CPU):", font=font_bold, fill=TEXT_MAIN)
        cpu_val = 340 + int(30 * math.sin(progress * 10))
        cpu_pct = cpu_val / 500.0
        draw.rounded_rectangle([80, 535, 900, 570], radius=6, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.rounded_rectangle([82, 537, 82 + int(816 * cpu_pct), 568], radius=5, fill=CYAN)
        draw.text((850, 542), f"{cpu_val}m / 500m", font=font_bold, fill=TEXT_MAIN)

        draw.text((80, 595), "RAM Utilization (Limit: 512Mi RAM):", font=font_bold, fill=TEXT_MAIN)
        ram_val = 218 + int(20 * math.cos(progress * 8))
        ram_pct = ram_val / 512.0
        draw.rounded_rectangle([80, 625, 900, 660], radius=6, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.rounded_rectangle([82, 627, 82 + int(816 * ram_pct), 658], radius=5, fill=GREEN)
        draw.text((850, 632), f"{ram_val}Mi / 512Mi", font=font_bold, fill=TEXT_MAIN)

        draw.text((80, 685), "Execution Timeout Counter (Limit: 300s):", font=font_bold, fill=TEXT_MAIN)
        time_rem = max(0, 300 - int(progress * 72))
        time_pct = time_rem / 300.0
        draw.rounded_rectangle([80, 715, 900, 750], radius=6, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
        draw.rounded_rectangle([82, 717, 82 + int(816 * time_pct), 748], radius=5, fill=PURPLE)
        draw.text((850, 722), f"{time_rem}s remaining", font=font_bold, fill=TEXT_MAIN)

        zeroclaw_logs = [
            "[10:00:12] INFO  zeroclaw: Spawning AST Parser for crates/factory-mcp-server...",
            "[10:00:15] INFO  zeroclaw: Analyzing tree-sitter AST nodes for Spec-Kit Tool...",
            "[10:00:18] SURGERY Applying surgical AST mutations to spec_kit_tool.rs...",
            "[10:00:22] SURGERY Injecting ResourceBounds validator (CPU: 500m, RAM: 512Mi)...",
            "[10:00:28] COMPILE Running 'cargo check --workspace' inside gVisor sandbox...",
            "[10:00:35] COMPILE Workspace compiled successfully in 7.2s with 0 warnings.",
            "[10:00:42] TEST    Executing cargo test -p factory-mcp-server...",
            "[10:00:50] TEST    test tools::spec_kit_tool::tests::test_bounds_enforcement ... ok",
            "[10:00:58] TEST    test tools::spec_kit_tool::tests::test_nhi_verification ... ok",
            "[10:01:05] SUCCESS All 18 unit tests passed inside sandboxed gVisor container!",
            "[10:01:10] SAST    Handing off artifact bundle to AuditorAgent for security audit..."
        ]
        num_show = max(1, min(len(zeroclaw_logs), int(progress * len(zeroclaw_logs) * 1.4)))
        y_pos = 450
        for idx in range(num_show):
            line = zeroclaw_logs[idx]
            col = GREEN if "SUCCESS" in line or "ok" in line else PURPLE if "SURGERY" in line else YELLOW if "COMPILE" in line or "TEST" in line else CYAN if "SAST" in line else TEXT_MAIN
            draw.text((1020, y_pos), line, font=font_mono, fill=col)
            y_pos += 38

    elif sec_num == 4:
        if progress >= 0.4:
            draw.rounded_rectangle([80, 500, 900, 890], radius=8, fill=(13, 17, 23), outline=BORDER_COLOR, width=1)
            draw.text((100, 515), "@lgcorzo (Human Architect)", font=font_bold, fill=CYAN)
            draw.text((780, 515), "10:04 AM", font=font_small, fill=TEXT_MUTED)
            draw.text((100, 545), "@darkgravity /refine Add strict validation for empty payload strings", font=font_heading, fill=YELLOW)

            draw.rounded_rectangle([100, 580, 160, 610], radius=12, fill=(22, 35, 50), outline=CYAN, width=1)
            draw.text((115, 588), "👀  1", font=font_small, fill=TEXT_MAIN)

            draw.rounded_rectangle([100, 620, 880, 870], radius=6, fill=(18, 24, 32), outline=BLUE_GLOW, width=1)
            draw.text((120, 635), "darkgravity-bot (Factory Intelligence)", font=font_bold, fill=PURPLE)

            refine_lines = [
                "⚡ Triggering ZeroClaw Refinement Loop in gVisor sandbox...",
                "• Surgical AST edit applied to spec_kit_tool.rs",
                "• Unit tests re-executed: 19/19 Passed",
                "• Refinement Commit pushed: e9f8a12 ('refactor: add payload string check')",
                "• SAST security check re-verified: 0 Vulnerabilities"
            ]
            y_pos = 665
            for line in refine_lines:
                col = GREEN if "Passed" in line or "pushed" in line else YELLOW if "Triggering" in line else TEXT_MAIN
                draw.text((120, y_pos), line, font=font_body, fill=col)
                y_pos += 38

    # Dynamic Caption Overlay
    caption_text = get_caption(sec_num, sec_time)
    draw_caption(draw, caption_text)

    return image.tobytes()

def main():
    print("Starting Screencast Render Pipeline (Cached Base Image Accelerated)...")
    os.makedirs(os.path.dirname(OUTPUT_VIDEO), exist_ok=True)

    print("Building audio narration track...")
    audio_concat_cmd = [
        "ffmpeg", "-y",
        "-i", "docs/assets/audio/sec1.mp3",
        "-i", "docs/assets/audio/sec2.mp3",
        "-i", "docs/assets/audio/sec3.mp3",
        "-i", "docs/assets/audio/sec4.mp3",
        "-i", "docs/assets/audio/sec5.mp3",
        "-filter_complex",
        "[0:a]apad=whole_dur=45[a1];"
        "[1:a]apad=whole_dur=75[a2];"
        "[2:a]apad=whole_dur=75[a3];"
        "[3:a]apad=whole_dur=60[a4];"
        "[4:a]apad=whole_dur=45[a5];"
        "[a1][a2][a3][a4][a5]concat=n=5:v=0:a=1[a]",
        "-map", "[a]",
        "docs/assets/audio/full_narration.wav"
    ]
    subprocess.run(audio_concat_cmd, check=True)

    ffmpeg_cmd = [
        "ffmpeg", "-y",
        "-f", "rawvideo",
        "-vcodec", "rawvideo",
        "-s", f"{WIDTH}x{HEIGHT}",
        "-pix_fmt", "rgb24",
        "-r", str(RENDER_FPS),
        "-i", "-", # Stdin pipe
        "-i", "docs/assets/audio/full_narration.wav",
        "-c:v", "libx264",
        "-preset", "ultrafast",
        "-crf", "18",
        "-r", "60", # Output framerate 60 FPS
        "-pix_fmt", "yuv420p",
        "-c:a", "aac",
        "-b:a", "192k",
        "-shortest",
        OUTPUT_VIDEO
    ]

    ffmpeg_proc = subprocess.Popen(ffmpeg_cmd, stdin=subprocess.PIPE)

    print(f"Rendering {TOTAL_FRAMES} frames (1080p @ 60fps output)...")

    for f_idx in range(TOTAL_FRAMES):
        if f_idx % 900 == 0:
            print(f"  Frame {f_idx}/{TOTAL_FRAMES} ({(f_idx/TOTAL_FRAMES)*100:.1f}% complete)...")
        raw_bytes = render_frame_bytes(f_idx)
        ffmpeg_proc.stdin.write(raw_bytes)

    ffmpeg_proc.stdin.close()
    ffmpeg_proc.wait()

    print(f"Screencast render COMPLETE! Output saved to {OUTPUT_VIDEO}")

if __name__ == "__main__":
    main()
