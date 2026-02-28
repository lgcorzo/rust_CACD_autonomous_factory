"""Security Review tool — analyzes code diffs against OWASP patterns and R2R RAG."""

from __future__ import annotations

import json
import re
import typing as T

import httpx
import litellm

from autogen_team.infrastructure.services.mcp_service import MCPService

# OWASP Top 10 pattern matchers (simplified regex patterns)
OWASP_PATTERNS: T.List[T.Dict[str, str]] = [
    {
        "rule": "A03:Injection - SQL",
        "pattern": r"""(?:execute|cursor\.execute|raw\s*\()\s*\(?.*?[\"'].*?%s|format\(|f[\"']""",
        "severity": "high",
        "description": "Potential SQL injection via string formatting in query.",
    },
    {
        "rule": "A03:Injection - Command",
        "pattern": r"(?:os\.system|subprocess\.call|subprocess\.Popen)\s*\(\s*[f\"']",
        "severity": "high",
        "description": "Potential command injection via formatted string.",
    },
    {
        "rule": "A02:Crypto - Hardcoded Secret",
        "pattern": r"""(?:password|secret|api_key|token)\s*=\s*[\"'][^\"']{8,}[\"']""",
        "severity": "medium",
        "description": "Possible hardcoded secret or credential.",
    },
    {
        "rule": "A05:Security Misconfiguration",
        "pattern": r"(?:debug\s*=\s*True|DEBUG\s*=\s*True|verify\s*=\s*False)",
        "severity": "medium",
        "description": "Debug mode enabled or TLS verification disabled.",
    },
    {
        "rule": "A07:XSS - Unsafe HTML",
        "pattern": r"(?:innerHTML|dangerouslySetInnerHTML|Markup\(|safe\s*=\s*True)",
        "severity": "medium",
        "description": "Potential cross-site scripting via unsafe HTML rendering.",
    },
    {
        "rule": "A08:Deserialization",
        "pattern": r"(?:pickle\.loads|yaml\.load\s*\([^)]*\)|eval\s*\(|exec\s*\()",
        "severity": "high",
        "description": "Unsafe deserialization or code execution.",
    },
]


def _scan_owasp_patterns(diff: str) -> T.List[T.Dict[str, str]]:
    """Scan diff against OWASP patterns.

    Args:
        diff: The code diff string to analyze.

    Returns:
        List of findings dicts with rule, severity, location, description.
    """
    findings: T.List[T.Dict[str, str]] = []
    lines = diff.split("\n")

    for i, line in enumerate(lines, start=1):
        # Only check added/modified lines
        if not line.startswith("+") and not line.startswith(" "):
            continue

        clean_line = line.lstrip("+").lstrip()

        for pattern_def in OWASP_PATTERNS:
            if re.search(pattern_def["pattern"], clean_line, re.IGNORECASE):
                findings.append(
                    {
                        "rule": pattern_def["rule"],
                        "severity": pattern_def["severity"],
                        "location": f"line {i}",
                        "description": pattern_def["description"],
                    }
                )

    return findings


async def _query_r2r_security(diff: str, r2r_base_url: str) -> T.List[T.Dict[str, T.Any]]:
    """Query R2R RAG for security best practices relevant to the diff.

    Args:
        diff: Code diff to find context for.
        r2r_base_url: R2R API base URL.

    Returns:
        List of relevant security documents.
    """
    try:
        async with httpx.AsyncClient(base_url=r2r_base_url, timeout=httpx.Timeout(15.0)) as client:
            response = await client.post(
                "/v3/retrieval/search",
                json={
                    "query": f"security best practices for: {diff[:500]}",
                    "search_settings": {
                        "limit": 5,
                    },
                },
            )
            response.raise_for_status()
            data = response.json()
            results = data.get("results", {})
            return T.cast(
                T.List[T.Dict[str, T.Any]],
                results.get("chunk_search_results", []),
            )
    except (httpx.HTTPError, Exception):
        return []


async def security_review(diff: str) -> T.Dict[str, T.Any]:
    """Analyze code diffs against OWASP patterns and R2R RAG security knowledge.

    Args:
        diff: The code diff string to review.

    Returns:
        Dict with status (approved/rejected) and findings list.
    """
    if not diff or not diff.strip():
        return {"status": "approved", "findings": [], "note": "Empty diff"}

    mcp_service = MCPService()
    system_prompt = mcp_service.get_prompt("security_review", "system")
    instruction_template = mcp_service.get_prompt("security_review", "instructions")

    # Step 1: OWASP pattern matching
    owasp_findings = _scan_owasp_patterns(diff)

    # Step 2: R2R RAG security context
    rag_context = await _query_r2r_security(diff, mcp_service.r2r_base_url)
    rag_text = "\n".join(doc.get("text", "")[:200] for doc in rag_context[:3])

    # Step 3: Optional Dynamic Analysis (Sandbox placeholder)
    # if mcp_service.use_sandbox:
    #     sandbox_result = await run_dynamic_analysis(diff)
    #     ...

    # Step 3: LiteLLM synthesis
    prompt = instruction_template.format(
        diff=diff[:3000],
        owasp_findings=json.dumps(owasp_findings, indent=2),
        rag_context=f"Relevant security best practices:\n{rag_text}" if rag_text else "",
    )

    try:
        response = await litellm.acompletion(
            model=mcp_service.litellm_model,
            messages=[
                {
                    "role": "system",
                    "content": system_prompt,
                },
                {"role": "user", "content": prompt},
            ],
            api_base=mcp_service.litellm_api_base,
            api_key=mcp_service.litellm_api_key,
            response_format={"type": "json_object"},
            temperature=0.0,
        )

        content = response.choices[0].message.content or "{}"
        llm_result = json.loads(content)
        additional = llm_result.get("additional_findings", [])
        all_findings = owasp_findings + additional
        verdict = llm_result.get("verdict", "approved")
    except (json.JSONDecodeError, Exception):
        all_findings = owasp_findings
        verdict = "rejected" if any(f["severity"] == "high" for f in owasp_findings) else "approved"

    # Override verdict if high-severity OWASP findings exist
    if any(f["severity"] == "high" for f in owasp_findings):
        verdict = "rejected"

    return {
        "status": verdict,
        "findings": all_findings,
    }
