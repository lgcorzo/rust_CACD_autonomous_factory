use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;
use uuid::Uuid;

/// Explicit evidence classification hierarchy (DG-RVP-014 §3.2).
/// Lower-level classes (e.g., `UNIT`) must never satisfy a higher-level requirement (e.g., `LIVE_RUNTIME`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceClass {
    Unit,
    Component,
    Integration,
    KubernetesAdmission,
    LiveRuntime,
    SecurityNegative,
    Chaos,
    ManualReview,
    Documentation,
    Mock,
}

impl EvidenceClass {
    pub fn from_str_canonical(s: &str) -> Option<Self> {
        match s.trim().to_ascii_lowercase().as_str() {
            "unit" => Some(Self::Unit),
            "component" => Some(Self::Component),
            "integration" => Some(Self::Integration),
            "kubernetes_admission" | "infrastructure" => Some(Self::KubernetesAdmission),
            "live_runtime" => Some(Self::LiveRuntime),
            "security_negative" => Some(Self::SecurityNegative),
            "chaos" => Some(Self::Chaos),
            "manual_review" => Some(Self::ManualReview),
            "documentation" | "performance" | "accessibility" => Some(Self::Documentation),
            "mock" => Some(Self::Mock),
            _ => None,
        }
    }

    /// Maximum allowed evidence age in days before it is considered stale (DG-RVP-014 §3.2).
    pub fn max_age_days(&self) -> i64 {
        match self {
            Self::LiveRuntime | Self::KubernetesAdmission | Self::Chaos => 30,
            Self::SecurityNegative | Self::Integration | Self::Component | Self::Unit => 90,
            Self::ManualReview | Self::Documentation => 180,
            Self::Mock => 0,
        }
    }

    /// Returns whether `candidate` can satisfy a requirement demanding `required`.
    /// A lower-level test must never satisfy a higher-level evidence requirement.
    pub fn satisfies(&self, required: Self) -> bool {
        if *self == Self::Mock {
            return false;
        }
        if *self == required {
            return true;
        }
        matches!(
            (*self, required),
            (Self::Component, Self::Unit)
                | (Self::Integration, Self::Unit | Self::Component)
                | (
                    Self::LiveRuntime,
                    Self::KubernetesAdmission | Self::Integration
                )
        )
    }
}

/// Returns the mandatory evidence class for a given requirement ID (DG-RVP-014 §3.2 & §3.4).
pub fn required_class_for_requirement(requirement_id: &str) -> Option<EvidenceClass> {
    match requirement_id {
        "FR-008" => Some(EvidenceClass::LiveRuntime),
        "FR-016" => Some(EvidenceClass::KubernetesAdmission),
        "FR-003" | "FR-004" | "FR-009" | "FR-010" | "FR-011" | "FR-012" | "FR-013" | "SC-001"
        | "SC-002" | "SC-003" => Some(EvidenceClass::Integration),
        "FR-005" | "FR-006" | "FR-007" | "FR-018" | "FR-019" | "FR-020" | "FR-021" | "SC-005"
        | "SC-006" => Some(EvidenceClass::Unit),
        "FR-014" | "FR-015" | "SC-004" => Some(EvidenceClass::Documentation),
        _ => None,
    }
}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generator_version: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_evidence_refs: Vec<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
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
    if let Some(started) = &record.started_at
        && DateTime::parse_from_rfc3339(started).is_err()
    {
        bail!("Invalid RFC3339 timestamp for started_at '{started}'");
    }
    if let Some(finished) = &record.finished_at
        && DateTime::parse_from_rfc3339(finished).is_err()
    {
        bail!("Invalid RFC3339 timestamp for finished_at '{finished}'");
    }
    for rel in &record.related_evidence_refs {
        if Uuid::parse_str(rel).is_err() {
            bail!("Invalid UUID in related_evidence_refs: '{rel}'");
        }
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

    let Some(parsed_class) = EvidenceClass::from_str_canonical(&record.verification_class) else {
        bail!("Invalid verification_class '{}'", record.verification_class);
    };

    if matches!(
        parsed_class,
        EvidenceClass::LiveRuntime | EvidenceClass::KubernetesAdmission
    ) && record.deployment_revision.is_none()
    {
        bail!(
            "live_runtime / kubernetes_admission evidence '{}' must include a non-null deployment_revision",
            record.evidence_id
        );
    }

    if matches!(
        parsed_class,
        EvidenceClass::LiveRuntime | EvidenceClass::KubernetesAdmission | EvidenceClass::Chaos
    ) {
        if matches!(record.environment.kind.as_str(), "local" | "ci") {
            bail!(
                "live_runtime / kubernetes_admission evidence '{}' cannot have environment.kind='{}' (must be 'cluster', 'production', 'staging', or 'development')",
                record.evidence_id,
                record.environment.kind
            );
        }
        let tool_lower = record.method.tool.trim().to_ascii_lowercase();
        let cmd_lower = record.method.command_or_test.trim().to_ascii_lowercase();
        if tool_lower == "cargo-test"
            || tool_lower == "pytest"
            || cmd_lower.starts_with("cargo test")
            || cmd_lower.starts_with("pytest")
            || cmd_lower.contains("mock")
        {
            bail!(
                "live_runtime / kubernetes_admission evidence '{}' cannot be generated from unit/integration test runner or mock tool='{}' command='{}'",
                record.evidence_id,
                record.method.tool,
                record.method.command_or_test
            );
        }
    }

    if parsed_class == EvidenceClass::LiveRuntime {
        let has_structured_runtime_artifact = record.artifacts.iter().any(|a| {
            let uri_lower = a.uri.to_ascii_lowercase();
            let media_lower = a
                .media_type
                .as_deref()
                .unwrap_or_default()
                .to_ascii_lowercase();
            !uri_lower.ends_with(".md")
                && !uri_lower.ends_with(".rs")
                && !uri_lower.ends_with(".py")
                && (uri_lower.ends_with(".json")
                    || uri_lower.ends_with(".log")
                    || media_lower == "application/json")
        });
        if !has_structured_runtime_artifact {
            bail!(
                "live_runtime evidence '{}' cannot rely solely on Markdown or source files; it must include at least one structured runtime telemetry artifact (.json or .log)",
                record.evidence_id
            );
        }
    }

    if parsed_class == EvidenceClass::SecurityNegative {
        let cmd_lower = record.method.command_or_test.trim().to_ascii_lowercase();
        if cmd_lower.contains("mock") || cmd_lower.contains("test_double") {
            bail!(
                "security_negative evidence '{}' cannot be certified by a mock or test double",
                record.evidence_id
            );
        }
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
    if parsed_class == EvidenceClass::Mock && record.outcome == "passed" {
        bail!(
            "Evidence '{}' with verification_class 'mock' cannot have outcome 'passed' (must be 'mocked')",
            record.evidence_id
        );
    }

    // Enforce exit_code / outcome coherence (DG-RVP-014 §3.4 & §3.5 #6, #10)
    if record.outcome == "passed" {
        match record.exit_code {
            None => {
                bail!(
                    "Evidence '{}' has outcome='passed' but missing exit_code (missing test result must be 'not_executed', not 'passed')",
                    record.evidence_id
                );
            }
            Some(code) if code != 0 => {
                bail!(
                    "Evidence '{}' claims outcome='passed' but captured non-zero exit_code={}",
                    record.evidence_id,
                    code
                );
            }
            Some(_) => {}
        }
    }
    if record.outcome == "failed" && record.exit_code == Some(0) {
        bail!(
            "Evidence '{}' has outcome='failed' but exit_code=0",
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

    if matches!(state.status.as_str(), "IMPLEMENTED" | "TESTED" | "VERIFIED")
        && state
            .implementation_agent_id
            .as_deref()
            .unwrap_or_default()
            .trim()
            .is_empty()
    {
        bail!(
            "Requirement '{}' in '{}' status requires non-empty implementation_agent_id",
            state.requirement_id,
            state.status
        );
    }

    if matches!(state.status.as_str(), "TESTED" | "VERIFIED") && state.evidence_refs.is_empty() {
        bail!(
            "Requirement '{}' in '{}' status requires non-empty qualifying evidence_refs",
            state.requirement_id,
            state.status
        );
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

        // Cryptographic verification if signature and public_key are provided
        if let (Some(pk_hex), Some(sig_hex)) = (&verifier.public_key, &verifier.signature) {
            use ed25519_dalek::{Signature, Verifier, VerifyingKey};
            let pk_bytes = hex::decode(pk_hex).map_err(|e| {
                anyhow::anyhow!("Invalid hex encoding for verifier public_key: {}", e)
            })?;
            let pk_array: [u8; 32] = pk_bytes
                .try_into()
                .map_err(|_| anyhow::anyhow!("verifier public_key must be exactly 32 bytes"))?;
            let verifying_key = VerifyingKey::from_bytes(&pk_array)
                .map_err(|e| anyhow::anyhow!("Invalid Ed25519 public key: {}", e))?;

            let sig_bytes = hex::decode(sig_hex).map_err(|e| {
                anyhow::anyhow!("Invalid hex encoding for verifier signature: {}", e)
            })?;
            let sig_array: [u8; 64] = sig_bytes
                .try_into()
                .map_err(|_| anyhow::anyhow!("verifier signature must be exactly 64 bytes"))?;
            let signature = Signature::from_bytes(&sig_array);

            let canonical_payload = format!(
                "{}:{}:{}:{}",
                state.requirement_id,
                verifier.id,
                verifier.verified_at,
                verifier.evidence_refs.join(",")
            );

            verifying_key
                .verify(canonical_payload.as_bytes(), &signature)
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Cryptographic signature verification failed for verifier '{}': {}",
                        verifier.id,
                        e
                    )
                })?;
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
            // Enforce maximum evidence age (DG-RVP-014 §3.2 & §3.4)
            let collected_dt = DateTime::parse_from_rfc3339(&ev.collected_at)
                .map_err(|e| anyhow::anyhow!("Invalid collected_at '{}': {}", ev.collected_at, e))?
                .with_timezone(&Utc);
            let verified_dt = DateTime::parse_from_rfc3339(&verifier.verified_at)
                .map_err(|e| {
                    anyhow::anyhow!("Invalid verified_at '{}': {}", verifier.verified_at, e)
                })?
                .with_timezone(&Utc);
            let ev_class = EvidenceClass::from_str_canonical(&ev.verification_class)
                .ok_or_else(|| anyhow::anyhow!("Invalid class '{}'", ev.verification_class))?;
            let age_days = (verified_dt - collected_dt).num_days();
            if age_days > ev_class.max_age_days() {
                bail!(
                    "Stale evidence '{}': collected {} days before verification (max allowed for {:?} is {} days)",
                    ev.evidence_id,
                    age_days,
                    ev_class,
                    ev_class.max_age_days()
                );
            }
            matched_records.push(ev);
        }

        // Enforce requirement-specific minimum evidence class hierarchy (DG-RVP-014 §3.2, §3.4, §3.5 #9)
        if let Some(mandatory_class) = required_class_for_requirement(&state.requirement_id) {
            let satisfied = matched_records.iter().any(|ev| {
                ev.outcome == "passed"
                    && EvidenceClass::from_str_canonical(&ev.verification_class)
                        .map(|c| c.satisfies(mandatory_class))
                        .unwrap_or(false)
            });
            if !satisfied {
                bail!(
                    "Requirement '{}' demands minimum evidence class '{:?}', which cannot be satisfied by lower-level evidence {:?}",
                    state.requirement_id,
                    mandatory_class,
                    matched_records
                        .iter()
                        .map(|e| e.verification_class.as_str())
                        .collect::<Vec<_>>()
                );
            }
        }

        for req_class in &policy.required_classes {
            let target_cls = EvidenceClass::from_str_canonical(req_class);
            let has_match = matched_records.iter().any(|ev| {
                ev.outcome == "passed"
                    && match (
                        EvidenceClass::from_str_canonical(&ev.verification_class),
                        target_cls,
                    ) {
                        (Some(cand), Some(req)) => cand.satisfies(req),
                        _ => &ev.verification_class == req_class,
                    }
            });
            if !has_match {
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
