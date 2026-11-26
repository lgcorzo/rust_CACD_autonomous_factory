use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::process::Command;

/// Represents an isolated git worktree checked out for a single Swarm execution lane.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneWorktree {
    pub mission_id: String,
    pub lane_id: String,
    pub branch_name: String,
    pub path: PathBuf,
}

/// Provisions, commits, lists, and prunes isolated per-lane git worktrees.
#[derive(Debug, Clone)]
pub struct WorktreeManager {
    repo_root: PathBuf,
    worktree_root: PathBuf,
}

impl WorktreeManager {
    /// Creates a `WorktreeManager` bound to `repo_root` storing lane checkouts under `worktree_root`.
    pub fn new(repo_root: impl Into<PathBuf>, worktree_root: impl Into<PathBuf>) -> Self {
        Self {
            repo_root: repo_root.into(),
            worktree_root: worktree_root.into(),
        }
    }

    /// Constructs a `WorktreeManager` from `WORKTREE_REPO_ROOT` and `WORKTREE_ROOT` env vars.
    pub fn from_env() -> Self {
        let repo_root = std::env::var("WORKTREE_REPO_ROOT")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        let worktree_root = std::env::var("WORKTREE_ROOT")
            .ok()
            .filter(|v| !v.trim().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("/tmp/dg-worktrees"));
        Self::new(repo_root, worktree_root)
    }

    /// Validates an identifier segment used inside filesystem paths and git branch names.
    pub fn validate_identifier(label: &str, value: &str) -> anyhow::Result<()> {
        if value.is_empty()
            || value == "."
            || value == ".."
            || value.starts_with('-')
            || value.contains("..")
            || value.contains('/')
            || value.contains('\\')
            || value.contains('\0')
            || !value
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            anyhow::bail!("Invalid {} for worktree isolation: '{}'", label, value);
        }
        Ok(())
    }

    /// Computes the canonical ephemeral branch name `swarm/<mission_id>/<lane_id>`.
    pub fn lane_branch_name(mission_id: &str, lane_id: &str) -> anyhow::Result<String> {
        Self::validate_identifier("mission_id", mission_id)?;
        Self::validate_identifier("lane_id", lane_id)?;
        Ok(format!("swarm/{}/{}", mission_id, lane_id))
    }

    fn mission_dir(&self, mission_id: &str) -> anyhow::Result<PathBuf> {
        Self::validate_identifier("mission_id", mission_id)?;
        Ok(self.worktree_root.join(mission_id))
    }

    fn resolve_lane_path(&self, mission_id: &str, lane_id: &str) -> anyhow::Result<PathBuf> {
        Self::validate_identifier("mission_id", mission_id)?;
        Self::validate_identifier("lane_id", lane_id)?;
        std::fs::create_dir_all(&self.worktree_root).with_context(|| {
            format!(
                "Failed to create worktree root directory {}",
                self.worktree_root.display()
            )
        })?;
        let canonical_root = std::fs::canonicalize(&self.worktree_root)?;
        let mission_dir = canonical_root.join(mission_id);
        std::fs::create_dir_all(&mission_dir)?;
        let canonical_mission = std::fs::canonicalize(&mission_dir)?;
        if !canonical_mission.starts_with(&canonical_root) {
            anyhow::bail!("Worktree mission directory escaped worktree root");
        }
        Ok(canonical_mission.join(lane_id))
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
            anyhow::bail!(
                "git {:?} failed in {}: {}",
                args,
                cwd.display(),
                stderr.trim()
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Provisions an isolated git worktree on ephemeral branch `swarm/<mission_id>/<lane_id>`.
    pub async fn provision_lane_worktree(
        &self,
        mission_id: &str,
        lane_id: &str,
        base_ref: Option<&str>,
    ) -> anyhow::Result<LaneWorktree> {
        let branch_name = Self::lane_branch_name(mission_id, lane_id)?;
        let lane_path = self.resolve_lane_path(mission_id, lane_id)?;

        let start_point = base_ref.unwrap_or("HEAD");
        if start_point.starts_with('-') || start_point.contains('\0') {
            anyhow::bail!("Invalid git base ref '{}'", start_point);
        }

        if lane_path.exists() {
            let _ = self.prune_lane_worktree(mission_id, lane_id).await;
        }

        let lane_path_str = lane_path.to_string_lossy().to_string();
        Self::run_git(
            &self.repo_root,
            &[
                "worktree",
                "add",
                "-B",
                &branch_name,
                &lane_path_str,
                start_point,
            ],
        )
        .await?;

        Ok(LaneWorktree {
            mission_id: mission_id.to_string(),
            lane_id: lane_id.to_string(),
            branch_name,
            path: lane_path,
        })
    }

    /// Stages and commits modified files inside an isolated lane worktree, returning commit SHA.
    pub async fn commit_lane_changes(
        &self,
        lane: &LaneWorktree,
        message: &str,
    ) -> anyhow::Result<Option<String>> {
        if !lane.path.exists() {
            anyhow::bail!("Lane worktree path does not exist: {}", lane.path.display());
        }
        let status = Self::run_git(&lane.path, &["status", "--porcelain"]).await?;
        if status.trim().is_empty() {
            return Ok(None);
        }

        Self::run_git(&lane.path, &["add", "-A"]).await?;
        Self::run_git(
            &lane.path,
            &[
                "-c",
                "user.name=DarkGravity-Swarm",
                "-c",
                "user.email=swarm@darkgravity.io",
                "commit",
                "-m",
                message,
            ],
        )
        .await?;
        let sha = Self::run_git(&lane.path, &["rev-parse", "HEAD"]).await?;
        Ok(Some(sha.trim().to_string()))
    }

    /// Lists currently checked-out lane worktrees belonging to `mission_id`.
    pub async fn list_mission_worktrees(
        &self,
        mission_id: &str,
    ) -> anyhow::Result<Vec<LaneWorktree>> {
        let mission_dir = self.mission_dir(mission_id)?;
        if !mission_dir.exists() {
            return Ok(Vec::new());
        }

        let mut lanes = Vec::new();
        for entry in std::fs::read_dir(&mission_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let lane_id = entry.file_name().to_string_lossy().to_string();
                if let Ok(branch_name) = Self::lane_branch_name(mission_id, &lane_id) {
                    lanes.push(LaneWorktree {
                        mission_id: mission_id.to_string(),
                        lane_id,
                        branch_name,
                        path: entry.path(),
                    });
                }
            }
        }
        lanes.sort_by(|a, b| a.lane_id.cmp(&b.lane_id));
        Ok(lanes)
    }

    /// Removes a single lane worktree directory and prunes git worktree metadata.
    pub async fn prune_lane_worktree(&self, mission_id: &str, lane_id: &str) -> anyhow::Result<()> {
        Self::validate_identifier("mission_id", mission_id)?;
        Self::validate_identifier("lane_id", lane_id)?;

        let canonical_root = match std::fs::canonicalize(&self.worktree_root) {
            Ok(root) => root,
            Err(_) => return Ok(()),
        };
        let lane_dir = canonical_root.join(mission_id).join(lane_id);
        let lane_dir_str = lane_dir.to_string_lossy().to_string();

        let _ = Self::run_git(
            &self.repo_root,
            &["worktree", "remove", "--force", &lane_dir_str],
        )
        .await;
        if lane_dir.exists() {
            std::fs::remove_dir_all(&lane_dir)?;
        }
        let _ = Self::run_git(&self.repo_root, &["worktree", "prune"]).await;
        Ok(())
    }

    /// Prunes every lane worktree under `mission_id` and cleans up its directory container.
    pub async fn prune_mission_worktrees(&self, mission_id: &str) -> anyhow::Result<usize> {
        let existing = self.list_mission_worktrees(mission_id).await?;
        let count = existing.len();
        for lane in &existing {
            self.prune_lane_worktree(mission_id, &lane.lane_id).await?;
        }
        let mission_dir = self.mission_dir(mission_id)?;
        if mission_dir.exists() {
            let _ = std::fs::remove_dir_all(&mission_dir);
        }
        let _ = Self::run_git(&self.repo_root, &["worktree", "prune"]).await;
        Ok(count)
    }

    pub fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn worktree_root(&self) -> &Path {
        &self.worktree_root
    }
}
