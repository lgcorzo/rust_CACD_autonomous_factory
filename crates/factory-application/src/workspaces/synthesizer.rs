use crate::workflows::circuit_breaker::compute_diff_hash;
use anyhow::Context;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use tokio::process::Command;

/// Input descriptor for a completed Swarm worker lane handed to Fan-In AST synthesis.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneBranchInput {
    pub lane_id: String,
    pub branch_name: String,
    pub task_id: String,
    #[serde(default)]
    pub touched_files: Vec<String>,
    pub succeeded: bool,
    #[serde(default)]
    pub diff_sha256: Option<String>,
}

/// Classification of cross-lane semantic/syntactic conflicts during Fan-In merge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConflictKind {
    SignatureDrift,
    DuplicateDefinition,
    Overlap,
    MergeConflict,
}

/// Single AST or merge conflict detected across parallel Swarm lanes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolConflict {
    pub kind: ConflictKind,
    pub symbol_name: String,
    pub file_path: String,
    pub lanes: Vec<String>,
    pub detail: String,
}

/// Outcome state produced by `AstSynthesizer::synthesize_branches`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SynthesisOutcome {
    Merged,
    ConflictedRetryable,
    OscillationHalted,
    EscalatedMaxAttempts,
    BlockedPartialFailure,
}

/// Extracted Rust or Python AST symbol signature used for cross-lane conflict inspection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AstSymbol {
    pub name: String,
    pub signature: String,
    pub language: String,
    pub file_path: String,
    pub lane_id: String,
}

/// Complete Fan-In synthesis report returned to `fan-in-synthesize`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SynthesisReport {
    pub mission_id: String,
    pub target_branch: String,
    pub outcome: SynthesisOutcome,
    pub merged_lanes: Vec<String>,
    pub blocked_lanes: Vec<String>,
    pub ast_conflicts: Vec<SymbolConflict>,
    pub resolution_attempt: u32,
    pub diff_sha256: Option<String>,
    pub merged_diff: String,
}

/// Deterministic AST conflict detector & branch synthesizer for Swarm Fan-In.
pub struct AstSynthesizer;

impl AstSynthesizer {
    /// Parses Rust and Python function/type definitions and normalized signatures.
    pub fn parse_symbols(lane_id: &str, file_path: &str, content: &str) -> Vec<AstSymbol> {
        let mut symbols = Vec::new();
        let is_python = file_path.ends_with(".py");
        let is_rust = file_path.ends_with(".rs");
        if !is_python && !is_rust {
            return symbols;
        }

        if is_rust {
            let fn_re = Regex::new(
                r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*(?:<[^>]*>)?\s*\(([^)]*)\)(?:\s*->\s*([^\{\n;]+))?",
            )
            .unwrap();
            for cap in fn_re.captures_iter(content) {
                let name = cap[1].to_string();
                let params = normalize_ws(&cap[2]);
                let ret = cap
                    .get(3)
                    .map(|m| normalize_ws(m.as_str()))
                    .unwrap_or_else(|| "()".to_string());
                symbols.push(AstSymbol {
                    name,
                    signature: format!("fn({}) -> {}", params, ret),
                    language: "rust".to_string(),
                    file_path: file_path.to_string(),
                    lane_id: lane_id.to_string(),
                });
            }

            let type_re = Regex::new(
                r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(struct|enum|trait)\s+([A-Za-z_][A-Za-z0-9_]*)",
            )
            .unwrap();
            for cap in type_re.captures_iter(content) {
                symbols.push(AstSymbol {
                    name: cap[2].to_string(),
                    signature: cap[1].to_string(),
                    language: "rust".to_string(),
                    file_path: file_path.to_string(),
                    lane_id: lane_id.to_string(),
                });
            }
        } else if is_python {
            let py_fn_re = Regex::new(
                r"(?m)^\s*(?:async\s+)?def\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^)]*)\)(?:\s*->\s*([^:\n]+))?\s*:",
            )
            .unwrap();
            for cap in py_fn_re.captures_iter(content) {
                let name = cap[1].to_string();
                let params = normalize_ws(&cap[2]);
                let ret = cap
                    .get(3)
                    .map(|m| normalize_ws(m.as_str()))
                    .unwrap_or_else(|| "None".to_string());
                symbols.push(AstSymbol {
                    name,
                    signature: format!("def({}) -> {}", params, ret),
                    language: "python".to_string(),
                    file_path: file_path.to_string(),
                    lane_id: lane_id.to_string(),
                });
            }

            let py_cls_re =
                Regex::new(r"(?m)^\s*class\s+([A-Za-z_][A-Za-z0-9_]*)(?:\(([^)]*)\))?\s*:")
                    .unwrap();
            for cap in py_cls_re.captures_iter(content) {
                let bases = cap
                    .get(2)
                    .map(|m| normalize_ws(m.as_str()))
                    .unwrap_or_default();
                symbols.push(AstSymbol {
                    name: cap[1].to_string(),
                    signature: format!("class({})", bases),
                    language: "python".to_string(),
                    file_path: file_path.to_string(),
                    lane_id: lane_id.to_string(),
                });
            }
        }

        symbols
    }

    /// Detects cross-lane `SignatureDrift`, `DuplicateDefinition`, and same-symbol `Overlap`.
    pub fn detect_symbol_conflicts(symbols: &[AstSymbol]) -> Vec<SymbolConflict> {
        let mut by_name: BTreeMap<String, Vec<&AstSymbol>> = BTreeMap::new();
        for sym in symbols {
            by_name.entry(sym.name.clone()).or_default().push(sym);
        }

        let mut conflicts = Vec::new();
        for (name, defs) in by_name {
            let distinct_lanes: BTreeSet<&str> = defs.iter().map(|d| d.lane_id.as_str()).collect();
            if distinct_lanes.len() < 2 {
                continue;
            }

            let lane_vec: Vec<String> = distinct_lanes.into_iter().map(str::to_string).collect();
            let distinct_files: BTreeSet<&str> =
                defs.iter().map(|d| d.file_path.as_str()).collect();
            let distinct_sigs: BTreeSet<&str> = defs.iter().map(|d| d.signature.as_str()).collect();

            if distinct_files.len() > 1 {
                conflicts.push(SymbolConflict {
                    kind: ConflictKind::DuplicateDefinition,
                    symbol_name: name.clone(),
                    file_path: defs[0].file_path.clone(),
                    lanes: lane_vec,
                    detail: format!(
                        "Symbol '{}' defined across multiple files by parallel lanes: {:?}",
                        name, distinct_files
                    ),
                });
            } else if distinct_sigs.len() > 1 {
                conflicts.push(SymbolConflict {
                    kind: ConflictKind::SignatureDrift,
                    symbol_name: name.clone(),
                    file_path: defs[0].file_path.clone(),
                    lanes: lane_vec,
                    detail: format!(
                        "Signature drift on '{}' in '{}': {:?}",
                        name, defs[0].file_path, distinct_sigs
                    ),
                });
            } else {
                conflicts.push(SymbolConflict {
                    kind: ConflictKind::Overlap,
                    symbol_name: name.clone(),
                    file_path: defs[0].file_path.clone(),
                    lanes: lane_vec,
                    detail: format!(
                        "Multiple lanes concurrently modified '{}' in '{}'",
                        name, defs[0].file_path
                    ),
                });
            }
        }

        conflicts
    }

    /// Returns `true` when the recent diff hash history exhibits an `A-B-A-B` flip-flop loop.
    pub fn detect_diff_oscillation(diff_history: &[String]) -> bool {
        if diff_history.len() < 4 {
            return false;
        }
        let n = diff_history.len();
        let a1 = &diff_history[n - 4];
        let b1 = &diff_history[n - 3];
        let a2 = &diff_history[n - 2];
        let b2 = &diff_history[n - 1];
        a1 == a2 && b1 == b2 && a1 != b1
    }

    /// Merges lane branches onto `mission-<mission_id>` cleanly after verifying AST compatibility.
    /// Fails closed on partial lane failure, `A-B-A-B` diff oscillation, or `> 3` resolution attempts.
    pub async fn synthesize_branches(
        repo_root: &Path,
        mission_id: &str,
        base_ref: &str,
        lanes: &[LaneBranchInput],
        resolution_attempt: u32,
        diff_history: &mut Vec<String>,
    ) -> anyhow::Result<SynthesisReport> {
        let target_branch = format!("mission-{}", mission_id);
        let blocked_lanes: Vec<String> = lanes
            .iter()
            .filter(|l| !l.succeeded)
            .map(|l| l.lane_id.clone())
            .collect();

        if !blocked_lanes.is_empty() {
            return Ok(SynthesisReport {
                mission_id: mission_id.to_string(),
                target_branch,
                outcome: SynthesisOutcome::BlockedPartialFailure,
                merged_lanes: Vec::new(),
                blocked_lanes,
                ast_conflicts: Vec::new(),
                resolution_attempt,
                diff_sha256: None,
                merged_diff: String::new(),
            });
        }

        if Self::detect_diff_oscillation(diff_history) {
            return Ok(SynthesisReport {
                mission_id: mission_id.to_string(),
                target_branch,
                outcome: SynthesisOutcome::OscillationHalted,
                merged_lanes: Vec::new(),
                blocked_lanes: Vec::new(),
                ast_conflicts: Vec::new(),
                resolution_attempt,
                diff_sha256: None,
                merged_diff: String::new(),
            });
        }

        if resolution_attempt > 3 {
            return Ok(SynthesisReport {
                mission_id: mission_id.to_string(),
                target_branch,
                outcome: SynthesisOutcome::EscalatedMaxAttempts,
                merged_lanes: Vec::new(),
                blocked_lanes: Vec::new(),
                ast_conflicts: Vec::new(),
                resolution_attempt,
                diff_sha256: None,
                merged_diff: String::new(),
            });
        }

        validate_git_ref(base_ref)?;
        validate_git_ref(&target_branch)?;

        // Inspect AST symbols across all changed files in each lane branch before merging.
        let mut parsed_symbols = Vec::new();
        for lane in lanes {
            validate_git_ref(&lane.branch_name)?;
            let range = format!("{}..{}", base_ref, lane.branch_name);
            let changed_out = run_git(repo_root, &["diff", "--name-only", &range]).await?;
            let mut files: Vec<String> = changed_out
                .lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect();
            if files.is_empty() {
                files = lane.touched_files.clone();
            }

            for rel_file in files {
                let spec = format!("{}:{}", lane.branch_name, rel_file);
                if let Ok(contents) = run_git(repo_root, &["show", &spec]).await {
                    parsed_symbols.extend(Self::parse_symbols(&lane.lane_id, &rel_file, &contents));
                }
            }
        }

        let ast_conflicts = Self::detect_symbol_conflicts(&parsed_symbols);
        if !ast_conflicts.is_empty() {
            let conflict_digest =
                compute_diff_hash(&serde_json::to_string(&ast_conflicts).unwrap_or_default());
            diff_history.push(conflict_digest.clone());
            let outcome = if Self::detect_diff_oscillation(diff_history) {
                SynthesisOutcome::OscillationHalted
            } else if resolution_attempt >= 3 {
                SynthesisOutcome::EscalatedMaxAttempts
            } else {
                SynthesisOutcome::ConflictedRetryable
            };
            return Ok(SynthesisReport {
                mission_id: mission_id.to_string(),
                target_branch,
                outcome,
                merged_lanes: Vec::new(),
                blocked_lanes: Vec::new(),
                ast_conflicts,
                resolution_attempt,
                diff_sha256: Some(conflict_digest),
                merged_diff: String::new(),
            });
        }

        run_git(repo_root, &["checkout", "-B", &target_branch, base_ref]).await?;

        let mut merged_lanes = Vec::new();
        let mut merge_conflicts = Vec::new();

        for lane in lanes {
            let msg = format!(
                "chore(swarm): Fan-In merge lane {} ({})",
                lane.lane_id, lane.task_id
            );
            let merge_res = run_git(
                repo_root,
                &["merge", "--no-ff", "-m", &msg, &lane.branch_name],
            )
            .await;

            match merge_res {
                Ok(_) => merged_lanes.push(lane.lane_id.clone()),
                Err(err) => {
                    let _ = run_git(repo_root, &["merge", "--abort"]).await;
                    merge_conflicts.push(SymbolConflict {
                        kind: ConflictKind::MergeConflict,
                        symbol_name: lane.branch_name.clone(),
                        file_path: lane.touched_files.first().cloned().unwrap_or_default(),
                        lanes: vec![lane.lane_id.clone()],
                        detail: err.to_string(),
                    });
                    break;
                }
            }
        }

        if !merge_conflicts.is_empty() {
            let conflict_digest =
                compute_diff_hash(&serde_json::to_string(&merge_conflicts).unwrap_or_default());
            diff_history.push(conflict_digest.clone());
            let outcome = if Self::detect_diff_oscillation(diff_history) {
                SynthesisOutcome::OscillationHalted
            } else if resolution_attempt >= 3 {
                SynthesisOutcome::EscalatedMaxAttempts
            } else {
                SynthesisOutcome::ConflictedRetryable
            };
            return Ok(SynthesisReport {
                mission_id: mission_id.to_string(),
                target_branch,
                outcome,
                merged_lanes: Vec::new(),
                blocked_lanes: Vec::new(),
                ast_conflicts: merge_conflicts,
                resolution_attempt,
                diff_sha256: Some(conflict_digest),
                merged_diff: String::new(),
            });
        }

        let range = format!("{}..HEAD", base_ref);
        let merged_diff = run_git(repo_root, &["diff", &range]).await?;
        let diff_sha256 = compute_diff_hash(&merged_diff);
        diff_history.push(diff_sha256.clone());

        if Self::detect_diff_oscillation(diff_history) {
            return Ok(SynthesisReport {
                mission_id: mission_id.to_string(),
                target_branch,
                outcome: SynthesisOutcome::OscillationHalted,
                merged_lanes: Vec::new(),
                blocked_lanes: Vec::new(),
                ast_conflicts: Vec::new(),
                resolution_attempt,
                diff_sha256: Some(diff_sha256),
                merged_diff,
            });
        }

        Ok(SynthesisReport {
            mission_id: mission_id.to_string(),
            target_branch,
            outcome: SynthesisOutcome::Merged,
            merged_lanes,
            blocked_lanes: Vec::new(),
            ast_conflicts: Vec::new(),
            resolution_attempt,
            diff_sha256: Some(diff_sha256),
            merged_diff,
        })
    }
}

fn normalize_ws(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn validate_git_ref(git_ref: &str) -> anyhow::Result<()> {
    if git_ref.is_empty() || git_ref.starts_with('-') || git_ref.contains('\0') {
        anyhow::bail!("Invalid git ref for synthesis: '{}'", git_ref);
    }
    Ok(())
}

async fn run_git(cwd: &Path, args: &[&str]) -> anyhow::Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .await
        .with_context(|| format!("Failed to spawn git {:?} in {}", args, cwd.display()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git {:?} failed: {}", args, stderr.trim());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
