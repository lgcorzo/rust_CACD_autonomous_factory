use anyhow::{Context, Result, bail};
use chrono::DateTime;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RevisionRef {
    pub repository: String,
    pub branch: Option<String>,
    pub commit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentRef {
    pub kind: String,
    pub identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VerificationMethod {
    pub tool: String,
    pub command_or_test: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceArtifact {
    pub uri: String,
    pub sha256: Option<String>,
    pub media_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CollectorIdentity {
    pub id: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRecord {
    pub schema_version: String,
    pub evidence_id: String,
    pub requirement_id: String,
    pub collected_at: String,
    pub source_revision: RevisionRef,
    pub deployment_revision: Option<RevisionRef>,
    pub environment: EnvironmentRef,
    pub verification_class: String,
    pub method: VerificationMethod,
    pub outcome: String,
    pub exit_code: Option<i32>,
    pub artifacts: Vec<EvidenceArtifact>,
    pub collector: CollectorIdentity,
    pub limitations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StateTransition {
    pub from: Option<String>,
    pub to: String,
    pub at: String,
    pub actor_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VerifierIdentity {
    pub id: String,
    pub role: String,
    pub verified_at: String,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RequirementState {
    pub schema_version: String,
    pub requirement_id: String,
    pub status: String,
    pub implementation_agent_id: Option<String>,
    pub evidence_refs: Vec<String>,
    pub transitions: Vec<StateTransition>,
    pub verified_by: Option<VerifierIdentity>,
    pub invalidated_at: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct VerificationPolicyConfig {
    pub expected_source_commit: Option<String>,
    pub required_classes: Vec<String>,
}

fn validate_revision_ref(rev: &RevisionRef, field_name: &str) -> Result<()> {
    if rev.repository.trim().is_empty() {
        bail!("{field_name}.repository must not be empty");
    }
    let sha_re = Regex::new(r"^[a-fA-F0-9]{40,64}$").unwrap();
    if !sha_re.is_match(&rev.commit) {
        bail!(
            "{field_name}.commit '{}' must be a 40-64 character hexadecimal SHA",
            rev.commit
        );
    }
    Ok(())
}

/// Validates an `EvidenceRecord` against `contracts/evidence-record.schema.json` and
/// optionally checks local `file://` artifact SHA-256 digests when `repo_root` is provided.
pub fn validate_evidence_record(record: &EvidenceRecord, repo_root: Option<&Path>) -> Result<()> {
    if record.schema_version != "1.0" {
        bail!(
            "Unsupported evidence schema_version '{}', expected '1.0'",
            record.schema_version
        );
    }
    if Uuid::parse_str(&record.evidence_id).is_err() {
        bail!("Invalid UUID for evidence_id '{}'", record.evidence_id);
    }
    let req_re = Regex::new(r"^(FR|SC)-[0-9]{3}$").unwrap();
    if !req_re.is_match(&record.requirement_id) {
        bail!(
            "Invalid requirement_id '{}', expected ^(FR|SC)-[0-9]{{3}}$",
            record.requirement_id
        );
    }
    if DateTime::parse_from_rfc3339(&record.collected_at).is_err() {
        bail!(
            "Invalid RFC3339 timestamp for collected_at '{}'",
            record.collected_at
        );
    }

    validate_revision_ref(&record.source_revision, "source_revision")?;
    if let Some(dep) = &record.deployment_revision {
        validate_revision_ref(dep, "deployment_revision")?;
    }

    let valid_env_kinds = [
        "local",
        "ci",
        "development",
        "staging",
        "production",
        "cluster",
        "other",
    ];
    if !valid_env_kinds.contains(&record.environment.kind.as_str()) {
        bail!("Invalid environment.kind '{}'", record.environment.kind);
    }
    if record.environment.identifier.trim().is_empty() {
        bail!("environment.identifier must not be empty");
    }

    let valid_classes = [
        "mock",
        "unit",
        "integration",
        "infrastructure",
        "live_runtime",
        "documentation",
        "performance",
        "accessibility",
    ];
    if !valid_classes.contains(&record.verification_class.as_str()) {
        bail!("Invalid verification_class '{}'", record.verification_class);
    }
    if record.verification_class == "live_runtime" && record.deployment_revision.is_none() {
        bail!(
            "live_runtime evidence '{}' must include a non-null deployment_revision",
            record.evidence_id
        );
    }

    if record.method.tool.trim().is_empty() || record.method.command_or_test.trim().is_empty() {
        bail!("method.tool and method.command_or_test must not be empty");
    }

    let valid_outcomes = [
        "passed",
        "failed",
        "not_executed",
        "infrastructure_error",
        "timeout",
        "mocked",
        "unavailable",
    ];
    if !valid_outcomes.contains(&record.outcome.as_str()) {
        bail!("Invalid outcome '{}'", record.outcome);
    }
    if record.verification_class == "mock" && record.outcome == "passed" {
        bail!(
            "Evidence '{}' with verification_class 'mock' cannot have outcome 'passed' (must be 'mocked')",
            record.evidence_id
        );
    }

    if record.artifacts.is_empty() {
        bail!("artifacts must contain at least 1 item");
    }
    let sha256_re = Regex::new(r"^[a-fA-F0-9]{64}$").unwrap();
    for artifact in &record.artifacts {
        if artifact.uri.trim().is_empty() {
            bail!("artifact.uri must not be empty");
        }
        if let Some(sha) = &artifact.sha256 {
            if !sha256_re.is_match(sha) {
                bail!("Invalid artifact.sha256 '{}'", sha);
            }
            if let (Some(root), Some(rel_path)) = (repo_root, artifact.uri.strip_prefix("file://"))
            {
                let full_path = root.join(rel_path);
                let bytes = std::fs::read(&full_path).with_context(|| {
                    format!(
                        "Referenced evidence artifact does not exist: {}",
                        full_path.display()
                    )
                })?;
                let actual_sha = hex::encode(Sha256::digest(&bytes));
                if !actual_sha.eq_ignore_ascii_case(sha) {
                    bail!(
                        "Artifact SHA-256 mismatch for {}: expected {}, actual {}",
                        artifact.uri,
                        sha,
                        actual_sha
                    );
                }
            }
        }
    }

    let valid_roles = [
        "implementer",
        "test_runner",
        "independent_verifier",
        "automated_system",
        "operator",
    ];
    if record.collector.id.trim().is_empty()
        || !valid_roles.contains(&record.collector.role.as_str())
    {
        bail!(
            "Invalid collector identity id='{}', role='{}'",
            record.collector.id,
            record.collector.role
        );
    }

    for lim in &record.limitations {
        if lim.trim().is_empty() {
            bail!("limitations items must not be empty strings");
        }
    }

    Ok(())
}

fn is_allowed_transition(from: Option<&str>, to: &str) -> bool {
    matches!(
        (from, to),
        (None, "OPEN")
            | (Some("OPEN"), "IMPLEMENTED")
            | (Some("IMPLEMENTED"), "TESTED")
            | (Some("TESTED"), "VERIFIED")
            | (Some("VERIFIED"), "OPEN")
    )
}

/// Validates a `RequirementState` against `contracts/verification-state.schema.json`
/// and enforces transition history, independent verifier identity, and evidence sufficiency.
pub fn validate_requirement_state(
    state: &RequirementState,
    records: &[EvidenceRecord],
    policy: &VerificationPolicyConfig,
) -> Result<()> {
    if state.schema_version != "1.0" {
        bail!(
            "Unsupported requirement state schema_version '{}', expected '1.0'",
            state.schema_version
        );
    }
    let req_re = Regex::new(r"^(FR|SC)-[0-9]{3}$").unwrap();
    if !req_re.is_match(&state.requirement_id) {
        bail!("Invalid requirement_id '{}'", state.requirement_id);
    }
    let valid_statuses = ["OPEN", "IMPLEMENTED", "TESTED", "VERIFIED"];
    if !valid_statuses.contains(&state.status.as_str()) {
        bail!("Invalid status '{}'", state.status);
    }

    let mut seen_refs = HashSet::new();
    for r in &state.evidence_refs {
        if Uuid::parse_str(r).is_err() {
            bail!("Invalid UUID in evidence_refs: '{r}'");
        }
        if !seen_refs.insert(r.clone()) {
            bail!("Duplicate evidence_ref '{r}'");
        }
    }

    if let Some(inv) = &state.invalidated_at
        && DateTime::parse_from_rfc3339(inv).is_err()
    {
        bail!("Invalid RFC3339 timestamp for invalidated_at '{inv}'");
    }

    // Validate transition chain
    let mut prev_to: Option<&str> = None;
    for t in &state.transitions {
        if DateTime::parse_from_rfc3339(&t.at).is_err() {
            bail!("Invalid transition timestamp '{}'", t.at);
        }
        if t.actor_id.trim().is_empty() || t.reason.trim().is_empty() {
            bail!("Transition actor_id and reason must not be empty");
        }
        if !is_allowed_transition(t.from.as_deref(), &t.to) {
            bail!(
                "Forbidden requirement transition {:?} -> {} (direct IMPLEMENTED -> VERIFIED jump or invalid step is prohibited)",
                t.from,
                t.to
            );
        }
        if let Some(prev) = prev_to
            && t.from.as_deref() != Some(prev)
        {
            bail!(
                "Broken transition chain: previous transition ended at '{}', next started from {:?}",
                prev,
                t.from
            );
        }
        prev_to = Some(&t.to);
    }

    if let Some(last) = prev_to
        && last != state.status
    {
        bail!(
            "Current status '{}' does not match final transition target '{}'",
            state.status,
            last
        );
    }

    if state.status == "OPEN" && state.verified_by.is_some() {
        bail!("Requirement in OPEN status must have verified_by = null");
    }

    if state.status == "VERIFIED" {
        if state.invalidated_at.is_some() {
            bail!(
                "Requirement '{}' cannot remain VERIFIED when invalidated_at is set",
                state.requirement_id
            );
        }
        if state.evidence_refs.is_empty() || state.transitions.is_empty() {
            bail!(
                "Requirement '{}' in VERIFIED status must have non-empty evidence_refs and transitions",
                state.requirement_id
            );
        }
        let verifier = state.verified_by.as_ref().ok_or_else(|| {
            anyhow::anyhow!(
                "Requirement '{}' in VERIFIED status requires verified_by",
                state.requirement_id
            )
        })?;
        if verifier.id.trim().is_empty() || verifier.role != "independent_verifier" {
            bail!("verified_by must have non-empty id and role='independent_verifier'");
        }
        if DateTime::parse_from_rfc3339(&verifier.verified_at).is_err() {
            bail!("Invalid verified_at timestamp '{}'", verifier.verified_at);
        }
        if verifier.evidence_refs.is_empty() {
            bail!("verified_by.evidence_refs must not be empty");
        }
        if let Some(impl_id) = &state.implementation_agent_id
            && impl_id == &verifier.id
        {
            bail!(
                "Self-reported verification rejected: verified_by.id '{}' matches implementation_agent_id '{}'",
                verifier.id,
                impl_id
            );
        }

        // Also check transitions: the actor who performed OPEN -> IMPLEMENTED cannot be the independent verifier
        for t in &state.transitions {
            if t.to == "IMPLEMENTED" && t.actor_id == verifier.id {
                bail!(
                    "Self-reported verification rejected: independent verifier '{}' also authored the IMPLEMENTED transition",
                    verifier.id
                );
            }
        }

        // Cross-check referenced evidence records
        let mut matched_records = Vec::new();
        for ref_id in &state.evidence_refs {
            let ev = records
                .iter()
                .find(|r| &r.evidence_id == ref_id)
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Missing referenced evidence record '{}' for requirement '{}'",
                        ref_id,
                        state.requirement_id
                    )
                })?;
            validate_evidence_record(ev, None)?;
            if ev.requirement_id != state.requirement_id {
                bail!(
                    "Evidence '{}' belongs to '{}', not '{}'",
                    ev.evidence_id,
                    ev.requirement_id,
                    state.requirement_id
                );
            }
            if ev.verification_class == "mock" || ev.outcome != "passed" {
                bail!(
                    "Evidence '{}' has class='{}' outcome='{}'; only non-mock 'passed' evidence can satisfy VERIFIED status",
                    ev.evidence_id,
                    ev.verification_class,
                    ev.outcome
                );
            }
            if let Some(expected_sha) = &policy.expected_source_commit
                && !ev.source_revision.commit.eq_ignore_ascii_case(expected_sha)
            {
                bail!(
                    "Stale source revision in evidence '{}': commit '{}' does not match expected HEAD '{}'",
                    ev.evidence_id,
                    ev.source_revision.commit,
                    expected_sha
                );
            }
            matched_records.push(ev);
        }

        for req_class in &policy.required_classes {
            if !matched_records
                .iter()
                .any(|ev| &ev.verification_class == req_class && ev.outcome == "passed")
            {
                bail!(
                    "Requirement '{}' is missing passing evidence for required verification_class '{}'",
                    state.requirement_id,
                    req_class
                );
            }
        }
    }

    Ok(())
}
