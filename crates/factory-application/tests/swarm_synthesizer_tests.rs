use factory_application::agents::RustantAgent;
use factory_application::workspaces::synthesizer::{
    AstSynthesizer, ConflictKind, LaneBranchInput, SynthesisOutcome,
};
use factory_infrastructure::{MockMcpClient, MockR2rClient, MockSemanticaClient};
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};
use uuid::Uuid;

struct TempRepoGuard(PathBuf);

impl TempRepoGuard {
    fn new(prefix: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("{}-{}", prefix, Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("create temp dir");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempRepoGuard {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn git_cmd(repo: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(args)
        .current_dir(repo)
        .output()
        .expect("spawn git");
    assert!(
        out.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn setup_repo_with_lanes(files_per_branch: &[(&str, &str, &str)]) -> TempRepoGuard {
    let guard = TempRepoGuard::new("swarm-synth-repo");
    let repo = guard.path();
    git_cmd(repo, &["init", "-b", "main"]);
    git_cmd(repo, &["config", "user.email", "synth@darkgravity.io"]);
    git_cmd(repo, &["config", "user.name", "Synth Test"]);
    std::fs::create_dir_all(repo.join("src")).unwrap();
    std::fs::write(repo.join("src/lib.rs"), "pub mod seed;\n").unwrap();
    git_cmd(repo, &["add", "."]);
    git_cmd(repo, &["commit", "-m", "chore: initial seed"]);

    for (branch, rel_path, content) in files_per_branch {
        git_cmd(repo, &["checkout", "-b", branch, "main"]);
        let target = repo.join(rel_path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&target, content).unwrap();
        git_cmd(repo, &["add", "."]);
        git_cmd(repo, &["commit", "-m", &format!("feat: {}", branch)]);
        git_cmd(repo, &["checkout", "main"]);
    }

    guard
}

#[tokio::test]
async fn test_ast_synthesizer_clean_merge_of_parallel_lanes() {
    let repo = setup_repo_with_lanes(&[
        (
            "swarm/m1/lane-1",
            "src/alpha.rs",
            "pub fn compute_alpha(x: i32) -> i32 { x + 1 }\n",
        ),
        (
            "swarm/m1/lane-2",
            "src/beta.rs",
            "pub fn compute_beta(y: &str) -> usize { y.len() }\n",
        ),
    ]);

    let lanes = vec![
        LaneBranchInput {
            lane_id: "lane-1".to_string(),
            branch_name: "swarm/m1/lane-1".to_string(),
            task_id: "T002".to_string(),
            touched_files: vec!["src/alpha.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
        LaneBranchInput {
            lane_id: "lane-2".to_string(),
            branch_name: "swarm/m1/lane-2".to_string(),
            task_id: "T003".to_string(),
            touched_files: vec!["src/beta.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
    ];

    let mut diff_history = Vec::new();
    let report = AstSynthesizer::synthesize_branches(
        repo.path(),
        "m1",
        "main",
        &lanes,
        1,
        &mut diff_history,
    )
    .await
    .expect("clean parallel synthesis must succeed");

    assert_eq!(report.outcome, SynthesisOutcome::Merged);
    assert_eq!(report.target_branch, "mission-m1");
    assert_eq!(report.merged_lanes, vec!["lane-1", "lane-2"]);
    assert!(report.ast_conflicts.is_empty());
    assert!(report.diff_sha256.is_some());
    assert!(repo.path().join("src/alpha.rs").exists());
    assert!(repo.path().join("src/beta.rs").exists());
}

#[tokio::test]
async fn test_ast_synthesizer_detects_signature_drift_and_python_conflict() {
    let repo = setup_repo_with_lanes(&[
        (
            "swarm/m2/lane-1",
            "src/shared.rs",
            "pub fn reconcile(id: u64) -> bool { id > 0 }\n",
        ),
        (
            "swarm/m2/lane-2",
            "src/shared.rs",
            "pub fn reconcile(id: &str) -> usize { id.len() }\n",
        ),
    ]);

    let lanes = vec![
        LaneBranchInput {
            lane_id: "lane-1".to_string(),
            branch_name: "swarm/m2/lane-1".to_string(),
            task_id: "T002".to_string(),
            touched_files: vec!["src/shared.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
        LaneBranchInput {
            lane_id: "lane-2".to_string(),
            branch_name: "swarm/m2/lane-2".to_string(),
            task_id: "T003".to_string(),
            touched_files: vec!["src/shared.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
    ];

    let mut diff_history = Vec::new();
    let report = AstSynthesizer::synthesize_branches(
        repo.path(),
        "m2",
        "main",
        &lanes,
        1,
        &mut diff_history,
    )
    .await
    .expect("synthesis report must be produced on AST conflict");

    assert_eq!(report.outcome, SynthesisOutcome::ConflictedRetryable);
    assert!(
        report
            .ast_conflicts
            .iter()
            .any(|c| c.kind == ConflictKind::SignatureDrift && c.symbol_name == "reconcile"),
        "Expected SignatureDrift for 'reconcile', got {:?}",
        report.ast_conflicts
    );

    // Verify Python signature drift detection too
    let mut py_symbols = AstSynthesizer::parse_symbols(
        "lane-1",
        "pkg/worker.py",
        "def dispatch_job(payload: dict) -> bool:\n    return True\n",
    );
    py_symbols.extend(AstSynthesizer::parse_symbols(
        "lane-2",
        "pkg/worker.py",
        "def dispatch_job(payload: str, retry: int) -> int:\n    return 0\n",
    ));
    let py_conflicts = AstSynthesizer::detect_symbol_conflicts(&py_symbols);
    assert!(
        py_conflicts
            .iter()
            .any(|c| c.kind == ConflictKind::SignatureDrift && c.symbol_name == "dispatch_job")
    );
}

#[tokio::test]
async fn test_ast_synthesizer_halts_on_max_3_attempts_and_abab_oscillation() {
    let repo = setup_repo_with_lanes(&[
        (
            "swarm/m3/lane-1",
            "src/api.rs",
            "pub fn execute() -> i32 { 1 }\n",
        ),
        (
            "swarm/m3/lane-2",
            "src/api.rs",
            "pub fn execute() -> String { String::new() }\n",
        ),
    ]);

    let lanes = vec![
        LaneBranchInput {
            lane_id: "lane-1".to_string(),
            branch_name: "swarm/m3/lane-1".to_string(),
            task_id: "T002".to_string(),
            touched_files: vec!["src/api.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
        LaneBranchInput {
            lane_id: "lane-2".to_string(),
            branch_name: "swarm/m3/lane-2".to_string(),
            task_id: "T003".to_string(),
            touched_files: vec!["src/api.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
    ];

    let mut diff_history = vec![
        "hash_a".to_string(),
        "hash_b".to_string(),
        "hash_a".to_string(),
    ];
    let report = AstSynthesizer::synthesize_branches(
        repo.path(),
        "m3",
        "main",
        &lanes,
        3,
        &mut diff_history,
    )
    .await
    .unwrap();
    assert_eq!(report.outcome, SynthesisOutcome::EscalatedMaxAttempts);

    // Direct A-B-A-B sequence trips OscillationHalted
    assert!(AstSynthesizer::detect_diff_oscillation(&[
        "sha-alpha".to_string(),
        "sha-beta".to_string(),
        "sha-alpha".to_string(),
        "sha-beta".to_string(),
    ]));
}

#[tokio::test]
async fn test_partial_lane_failure_blocks_without_pr_or_r2r_promotion() {
    let repo = setup_repo_with_lanes(&[(
        "swarm/m4/lane-1",
        "src/good.rs",
        "pub fn ok() -> bool { true }\n",
    )]);

    let lanes = vec![
        LaneBranchInput {
            lane_id: "lane-1".to_string(),
            branch_name: "swarm/m4/lane-1".to_string(),
            task_id: "T002".to_string(),
            touched_files: vec!["src/good.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
        LaneBranchInput {
            lane_id: "lane-2".to_string(),
            branch_name: "swarm/m4/lane-2".to_string(),
            task_id: "T003".to_string(),
            touched_files: vec!["src/bad.rs".to_string()],
            succeeded: false,
            diff_sha256: None,
        },
    ];

    let mut diff_history = Vec::new();
    let report = AstSynthesizer::synthesize_branches(
        repo.path(),
        "m4",
        "main",
        &lanes,
        1,
        &mut diff_history,
    )
    .await
    .unwrap();

    assert_eq!(report.outcome, SynthesisOutcome::BlockedPartialFailure);
    assert_eq!(report.blocked_lanes, vec!["lane-2"]);
    assert!(report.merged_lanes.is_empty());
}

#[tokio::test]
async fn test_rustant_synthesize_swarm_lanes_enforces_osr_semantica_and_sast() {
    let repo = setup_repo_with_lanes(&[
        (
            "swarm/m5/lane-1",
            "src/mod_a.rs",
            "pub fn alpha_step() -> bool { true }\n",
        ),
        (
            "swarm/m5/lane-2",
            "src/mod_b.rs",
            "pub fn beta_step() -> bool { true }\n",
        ),
    ]);

    let lanes = vec![
        LaneBranchInput {
            lane_id: "lane-1".to_string(),
            branch_name: "swarm/m5/lane-1".to_string(),
            task_id: "T002".to_string(),
            touched_files: vec!["src/mod_a.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
        LaneBranchInput {
            lane_id: "lane-2".to_string(),
            branch_name: "swarm/m5/lane-2".to_string(),
            task_id: "T003".to_string(),
            touched_files: vec!["src/mod_b.rs".to_string()],
            succeeded: true,
            diff_sha256: None,
        },
    ];

    let review_calls = Arc::new(AtomicU32::new(0));
    let review_calls_clone = Arc::clone(&review_calls);
    let mut mock_mcp = MockMcpClient::new();
    mock_mcp
        .expect_call_tool_json()
        .returning(move |tool, args| match tool {
            "security_review" => {
                review_calls_clone.fetch_add(1, Ordering::SeqCst);
                assert_eq!(args["inference_priority"], "Critical-Blocking");
                Ok(json!({
                    "is_error": false,
                    "content": [{ "text": "{\"score\": 9.1, \"status\": \"approved\", \"critical_count\": 0}" }]
                }))
            }
            _ => Ok(json!({ "is_error": false })),
        });

    let mut mock_r2r = MockR2rClient::new();
    mock_r2r
        .expect_search()
        .returning(|_| Ok("Architecture module mod_a and mod_b verified.".to_string()));
    mock_r2r.expect_push_osr_metric().times(1).returning(|m| {
        assert!(m.osr_value < 0.05);
        Ok(())
    });

    let mut mock_semantica = MockSemanticaClient::new();
    mock_semantica
        .expect_detect_conflicts()
        .times(1)
        .returning(|_| Ok(vec![]));

    let agent = RustantAgent::new(Arc::new(mock_mcp), Arc::new(mock_r2r));
    let mut diff_history = Vec::new();
    let result = agent
        .synthesize_swarm_lanes(
            repo.path(),
            "m5",
            "main",
            &lanes,
            Some(&mock_semantica),
            "Architecture module mod_a and mod_b verified.",
            &mut diff_history,
            1,
        )
        .await
        .expect("swarm synthesis gate must succeed");

    assert_eq!(result.report.outcome, SynthesisOutcome::Merged);
    assert!(result.osr_score < 0.05);
    assert!(result.security_score >= 8.0);
    assert_eq!(review_calls.load(Ordering::SeqCst), 1);
}
