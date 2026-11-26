use crate::Agent;
use crate::utils::osr::calculate_osr;
use crate::workspaces::synthesizer::{
    AstSynthesizer, ConflictKind, LaneBranchInput, SymbolConflict, SynthesisOutcome,
    SynthesisReport,
};
use async_trait::async_trait;
use factory_core::config::InferencePriorityClass;
use factory_infrastructure::{McpClient, R2rClient, SemanticaClient};
use serde_json::{Value, json};
use std::path::Path;
use std::sync::Arc;

/// Result produced by the Synthesizer role combining AST merge, Semantica, OSR, and SAST gates.
#[derive(Debug, Clone)]
pub struct SwarmSynthesisOutcome {
    pub report: SynthesisReport,
    pub security_score: f64,
    pub osr_score: f32,
    pub wiki_regenerations: u32,
    pub semantica_conflicts: Vec<String>,
}

pub struct RustantAgent {
    mcp_client: Arc<dyn McpClient>,
    r2r_client: Arc<dyn R2rClient>,
}

impl RustantAgent {
    pub fn new(mcp_client: Arc<dyn McpClient>, r2r_client: Arc<dyn R2rClient>) -> Self {
        Self {
            mcp_client,
            r2r_client,
        }
    }

    pub async fn discover_peer_skills(
        &self,
        a2a_client: &factory_infrastructure::ziti::ZitiA2aClient,
    ) -> anyhow::Result<Value> {
        a2a_client.discover_skills().await
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn delegate_a2a_task(
        &self,
        a2a_client: &factory_infrastructure::ziti::ZitiA2aClient,
        target: &str,
        skill_name: &str,
        arguments: Value,
        mission_id: &str,
        hop: u8,
        trace: Vec<String>,
    ) -> anyhow::Result<factory_infrastructure::ziti::A2aDispatchResponse> {
        let req = factory_infrastructure::ziti::A2aDispatchRequest {
            skill_name: skill_name.to_string(),
            arguments,
            headers: factory_infrastructure::ziti::A2aHeaders {
                target: target.to_string(),
                hop,
                trace,
                vtag_mission: Some(mission_id.to_string()),
                ..Default::default()
            },
            caller_id: "rustant-agent".to_string(),
        };
        a2a_client.dispatch(&req).await
    }

    pub async fn plan_mission(&self, mission_id: &str, goal: &str) -> anyhow::Result<Value> {
        tracing::info!(
            "[RustantAgent:{}] Planning mission for goal: {}",
            mission_id,
            goal
        );

        // 1. Context Pruning (R2R) - Gracefully fallback if R2R service is unreachable or unauthenticated
        let context = match self.r2r_client.search(goal).await {
            Ok(ctx) => ctx,
            Err(e) => {
                tracing::warn!(
                    "[RustantAgent:{}] R2R knowledge search unavailable ({:?}). Proceeding with empty context fallback.",
                    mission_id,
                    e
                );
                String::new()
            }
        };

        // 2. 6-Phase Spec-Kit sequence via MCP
        let phases = vec!["init", "specify", "plan", "execute", "verify", "git-commit"];

        for phase in phases {
            let mut args = vec![];

            // Inject R2R context into specify
            if phase == "specify" {
                args.push(format!("--context={}", context));
            }

            self.mcp_client
                .call_tool_json(
                    "invoke_spec_kit",
                    json!({
                        "command": phase,
                        "args": args
                    }),
                )
                .await?;
        }

        // 3. Parse generated artifacts
        let mut target_spec_dir = None;

        // Try reading .specify/init-options.json
        let config_str = std::fs::read_to_string(".specify/init-options.json").ok();
        let config: Option<serde_json::Value> =
            config_str.and_then(|s| serde_json::from_str(&s).ok());
        if let Some(specs_dir) = config
            .as_ref()
            .and_then(|c| c.get("specs_dir"))
            .and_then(|v| v.as_str())
        {
            let path = std::path::PathBuf::from(specs_dir);
            if path.exists() {
                target_spec_dir = Some(path);
            }
        }

        // Fall back to latest directory in specs/
        if target_spec_dir.is_none() {
            let entries = std::fs::read_dir("specs").ok();
            if let Some(entries) = entries {
                let mut dirs: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                dirs.sort_by_key(|dir| {
                    dir.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                });
                if let Some(latest) = dirs.last() {
                    target_spec_dir = Some(latest.path());
                }
            }
        }

        let mut parsed_spec = Value::Null;
        let mut parsed_plan = Value::Null;
        let mut parsed_tasks = Value::Null;

        let mut sdd_tasks = Vec::new();
        if let Some(spec_dir) = target_spec_dir {
            if let Ok(s) = std::fs::read_to_string(spec_dir.join("spec.md")) {
                parsed_spec = serde_json::from_str(&s).unwrap_or(json!(s));
            }
            if let Ok(s) = std::fs::read_to_string(spec_dir.join("plan.md")) {
                parsed_plan = serde_json::from_str(&s).unwrap_or(json!(s));
            }
            if let Ok(s) = std::fs::read_to_string(spec_dir.join("tasks.md")) {
                sdd_tasks = Self::parse_sdd_tasks(&s);
                parsed_tasks = serde_json::from_str(&s).unwrap_or(json!(s));
            }
        }

        if sdd_tasks.is_empty() {
            let goal_tasks = Self::parse_sdd_tasks(goal);
            if !goal_tasks.is_empty() {
                sdd_tasks = goal_tasks;
            }
        }

        if sdd_tasks.is_empty() {
            let goal_summary = goal.lines().next().unwrap_or(goal).trim();
            if parsed_spec.is_null() {
                parsed_spec = json!(format!(
                    "# Specification: {}\n\nMission ID: {}\nGoal: {}",
                    goal_summary, mission_id, goal
                ));
            }
            if parsed_plan.is_null() {
                parsed_plan = json!(format!(
                    "# Implementation Plan: {}\n\n1. Run AST/CRG comprehension\n2. Standardize SDD layout (.specify/, specs/, tests/)\n3. Generate ISO 42010 / OKF openwiki/Home.md & evidence/",
                    goal_summary
                ));
            }
            let enable_swarm_lanes = goal.contains("[P]")
                || goal.to_lowercase().contains("swarm mode")
                || goal.to_lowercase().contains("parallel lanes");
            sdd_tasks = if enable_swarm_lanes {
                vec![
                    factory_core::SddTaskItem {
                        id: "T001".to_string(),
                        description: format!(
                            "Standardize SDD & security baseline for {}",
                            goal_summary
                        ),
                        is_parallel: false,
                        dependencies: vec![],
                        target_files: vec![
                            ".specify/integration.json".to_string(),
                            "SECURITY.md".to_string(),
                        ],
                    },
                    factory_core::SddTaskItem {
                        id: "T002".to_string(),
                        description: format!(
                            "[P] Implement core deliverables for {}",
                            goal_summary
                        ),
                        is_parallel: true,
                        dependencies: vec!["T001".to_string()],
                        target_files: vec!["src/lib.rs".to_string()],
                    },
                    factory_core::SddTaskItem {
                        id: "T003".to_string(),
                        description: "[P] Generate ISO 42010 / OKF living Wiki (openwiki/Home.md & openwiki/AST_Architecture.md)"
                            .to_string(),
                        is_parallel: true,
                        dependencies: vec!["T001".to_string()],
                        target_files: vec![
                            "openwiki/Home.md".to_string(),
                            "openwiki/AST_Architecture.md".to_string(),
                        ],
                    },
                    factory_core::SddTaskItem {
                        id: "T004".to_string(),
                        description: "Verify synthesized lanes in gVisor sandbox and package mission evidence"
                            .to_string(),
                        is_parallel: false,
                        dependencies: vec!["T002".to_string(), "T003".to_string()],
                        target_files: vec![format!("evidence/missions/{}.md", mission_id)],
                    },
                ]
            } else {
                vec![
                    factory_core::SddTaskItem {
                        id: "T001".to_string(),
                        description: format!(
                            "Standardize SDD & security baseline for {}",
                            goal_summary
                        ),
                        is_parallel: false,
                        dependencies: vec![],
                        target_files: vec![
                            ".specify/integration.json".to_string(),
                            "SECURITY.md".to_string(),
                        ],
                    },
                    factory_core::SddTaskItem {
                        id: "T002".to_string(),
                        description: "Generate ISO 42010 / OKF living Wiki (openwiki/Home.md & openwiki/AST_Architecture.md)"
                            .to_string(),
                        is_parallel: false,
                        dependencies: vec!["T001".to_string()],
                        target_files: vec![
                            "openwiki/Home.md".to_string(),
                            "openwiki/AST_Architecture.md".to_string(),
                        ],
                    },
                    factory_core::SddTaskItem {
                        id: "T003".to_string(),
                        description: "Verify tests in gVisor sandbox and package mission evidence"
                            .to_string(),
                        is_parallel: false,
                        dependencies: vec!["T002".to_string()],
                        target_files: vec![format!("evidence/missions/{}.md", mission_id)],
                    },
                ]
            };
            if parsed_tasks.is_null() {
                parsed_tasks = json!(sdd_tasks);
            }
        }

        let sdd_plan = factory_core::SddMissionPlan {
            mission_id: mission_id.to_string(),
            spec_version: "1.0.0".to_string(),
            total_tasks: sdd_tasks.len(),
            tasks: sdd_tasks,
        };

        Ok(json!({
            "status": "spec_kit_planning_complete",
            "spec": parsed_spec,
            "plan": parsed_plan,
            "tasks": parsed_tasks,
            "sdd_plan": sdd_plan,
            "summary": "Implement the task described in the parsed plan and tasks artifacts."
        }))
    }

    pub fn parse_sdd_tasks(content: &str) -> Vec<factory_core::SddTaskItem> {
        let task_regex =
            regex::Regex::new(r"(?m)^-\s*\[[ xX]\]\s*(T\d+)\s*(\[P\])?\s*(?:\[[^\]]+\])?\s*(.+)$")
                .unwrap();
        let file_regex = regex::Regex::new(r"`([^`]+(?:\.[a-zA-Z0-9]+|Cargo\.toml))`").unwrap();

        let mut tasks = Vec::new();
        let mut previous_task_id: Option<String> = None;

        for cap in task_regex.captures_iter(content) {
            let id = cap.get(1).map_or("", |m| m.as_str()).to_string();
            let is_parallel = cap.get(2).is_some();
            let description = cap.get(3).map_or("", |m| m.as_str()).trim().to_string();

            let mut target_files = Vec::new();
            for file_cap in file_regex.captures_iter(&description) {
                if let Some(m) = file_cap.get(1) {
                    let path = m.as_str().to_string();
                    if !target_files.contains(&path) {
                        target_files.push(path);
                    }
                }
            }

            let mut dependencies = Vec::new();
            if let (false, Some(prev)) = (is_parallel, &previous_task_id) {
                dependencies.push(prev.clone());
            }
            previous_task_id = Some(id.clone());

            tasks.push(factory_core::SddTaskItem {
                id,
                description,
                is_parallel,
                dependencies,
                target_files,
            });
        }

        tasks
    }

    pub async fn review_mission(
        &self,
        mission_id: &str,
        mission_results: &str,
    ) -> anyhow::Result<Value> {
        tracing::info!("[RustantAgent:{}] Reviewing mission results", mission_id);

        let result = self
            .mcp_client
            .call_tool_json(
                "security_review",
                json!({
                    "mission_id": mission_id,
                    "artifacts": mission_results,
                    "inference_priority": InferencePriorityClass::CriticalBlocking.as_header_value(),
                }),
            )
            .await?;

        Ok(result)
    }

    /// Synthesizer role for Swarm Fan-In: merges lane branches via `AstSynthesizer`,
    /// checks Semantica conflicts, enforces OpenWiki `OSR < 5%` with up to 2 regenerations
    /// (`Background-Exploratory`), and validates Stage 4 SAST >= 8.0/10.0 (`Critical-Blocking`).
    #[allow(clippy::too_many_arguments)]
    pub async fn synthesize_swarm_lanes(
        &self,
        repo_root: &Path,
        mission_id: &str,
        base_ref: &str,
        lanes: &[LaneBranchInput],
        semantica: Option<&dyn SemanticaClient>,
        wiki_draft: &str,
        diff_history: &mut Vec<String>,
        resolution_attempt: u32,
    ) -> anyhow::Result<SwarmSynthesisOutcome> {
        let mut report = AstSynthesizer::synthesize_branches(
            repo_root,
            mission_id,
            base_ref,
            lanes,
            resolution_attempt,
            diff_history,
        )
        .await?;

        if report.outcome != SynthesisOutcome::Merged {
            return Ok(SwarmSynthesisOutcome {
                report,
                security_score: 0.0,
                osr_score: 1.0,
                wiki_regenerations: 0,
                semantica_conflicts: Vec::new(),
            });
        }

        let mut semantica_conflicts: Vec<String> = Vec::new();
        if let Some(sem_client) = semantica {
            let plan_payload = factory_infrastructure::semantica::MissionPlan {
                mission_id: mission_id.to_string(),
                title: format!("Swarm Fan-In synthesis for {}", mission_id),
                spec_content: report.merged_diff.clone(),
                constitution_rules: vec!["single-authoritative-synthesis".to_string()],
                proposed_tasks: lanes.iter().map(|l| l.task_id.clone()).collect(),
            };
            let detected = sem_client.detect_conflicts(&plan_payload).await?;
            if !detected.is_empty() {
                for conflict in &detected {
                    let formatted = format!(
                        "{} [{}]: {} ({})",
                        conflict.conflict_id,
                        conflict.severity,
                        conflict.description,
                        conflict.rule_violated
                    );
                    report.ast_conflicts.push(SymbolConflict {
                        kind: ConflictKind::Overlap,
                        symbol_name: conflict.rule_violated.clone(),
                        file_path: String::new(),
                        lanes: report.merged_lanes.clone(),
                        detail: formatted.clone(),
                    });
                    semantica_conflicts.push(formatted);
                }
                report.outcome = if resolution_attempt >= 3 {
                    SynthesisOutcome::EscalatedMaxAttempts
                } else {
                    SynthesisOutcome::ConflictedRetryable
                };
                return Ok(SwarmSynthesisOutcome {
                    report,
                    security_score: 0.0,
                    osr_score: 1.0,
                    wiki_regenerations: 0,
                    semantica_conflicts,
                });
            }
        }

        let r2r_text = self
            .r2r_client
            .search(&format!("mission {} architecture", mission_id))
            .await
            .unwrap_or_else(|_| wiki_draft.to_string());

        let mut current_wiki = wiki_draft.to_string();
        let mut wiki_regenerations = 0u32;
        let mut osr_score = calculate_osr(&current_wiki, &r2r_text);

        while osr_score >= 0.05 && wiki_regenerations < 2 {
            wiki_regenerations += 1;
            let regen = self
                .mcp_client
                .call_tool_json(
                    "invoke_spec_kit",
                    json!({
                        "command": "wiki-regenerate",
                        "mission_id": mission_id,
                        "inference_priority": InferencePriorityClass::BackgroundExploratory.as_header_value(),
                        "osr_score": osr_score,
                        "r2r_baseline": r2r_text,
                    }),
                )
                .await?;
            if let Some(updated) = regen["content"]
                .as_array()
                .and_then(|c| c.first())
                .and_then(|c| c["text"].as_str())
            {
                current_wiki = updated.to_string();
                osr_score = calculate_osr(&current_wiki, &r2r_text);
            } else {
                break;
            }
        }

        if osr_score >= 0.05 {
            anyhow::bail!(
                "OpenWiki OSR gate failed for mission '{}': {:.2}% >= 5.0% after {} regeneration(s)",
                mission_id,
                osr_score * 100.0,
                wiki_regenerations
            );
        }

        let metric = factory_core::OsrMetric {
            mission_id: mission_id.to_string(),
            osr_value: osr_score,
            wiki_commit_sha: report
                .diff_sha256
                .clone()
                .unwrap_or_else(|| "HEAD".to_string()),
            timestamp: chrono::Utc::now(),
        };
        self.r2r_client.push_osr_metric(&metric).await?;

        let review = self.review_mission(mission_id, &report.merged_diff).await?;
        if review["is_error"].as_bool().unwrap_or(false) {
            anyhow::bail!("Fan-In SAST security_review tool returned error");
        }

        let (security_score, critical_count, status) = review["content"]
            .as_array()
            .and_then(|c| c.first())
            .and_then(|c| c["text"].as_str())
            .and_then(|text| serde_json::from_str::<Value>(text).ok())
            .map(|parsed| {
                (
                    parsed["score"].as_f64().unwrap_or(0.0),
                    parsed["critical_count"].as_u64().unwrap_or(0),
                    parsed["status"].as_str().unwrap_or("rejected").to_string(),
                )
            })
            .unwrap_or((0.0, 1, "rejected".to_string()));

        if security_score < 8.0 || critical_count > 0 || status == "rejected" {
            anyhow::bail!(
                "Fan-In security review blocked promotion: score={:.2}, criticals={}, status={}",
                security_score,
                critical_count,
                status
            );
        }

        Ok(SwarmSynthesisOutcome {
            report,
            security_score,
            osr_score,
            wiki_regenerations,
            semantica_conflicts,
        })
    }
}

#[async_trait]
impl Agent for RustantAgent {
    fn name(&self) -> String {
        "rustant".to_string()
    }

    async fn execute(&self, task_description: &str) -> anyhow::Result<Value> {
        // Default to planning with a temporary ID if no specific action is provided
        self.plan_mission("default-id", task_description).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sdd_tasks_from_markdown() {
        let tasks_md = r#"
# Tasks: Pipeline Error Remediation
- [ ] T001 Add `regex = "1"` to `[workspace.dependencies]` in `Cargo.toml`
- [ ] T002 [P] Create empty module file `crates/factory-infrastructure/src/pipeline_classifier.rs` with module doc comment
- [ ] T003 [P] Create empty module file `crates/factory-application/src/workflows/pipeline_remediation.rs`
- [ ] T006 [P] Add `ErrorCategory` enum to `crates/factory-core/src/lib.rs` (per data-model.md)
"#;

        let parsed = RustantAgent::parse_sdd_tasks(tasks_md);
        assert_eq!(parsed.len(), 4);
        assert_eq!(parsed[0].id, "T001");
        assert!(!parsed[0].is_parallel);
        assert!(parsed[0].target_files.contains(&"Cargo.toml".to_string()));

        assert_eq!(parsed[1].id, "T002");
        assert!(parsed[1].is_parallel);
        assert!(
            parsed[1]
                .target_files
                .contains(&"crates/factory-infrastructure/src/pipeline_classifier.rs".to_string())
        );
    }
}
