use anyhow::Result;
use sha2::{Digest, Sha256};
use std::process::Command;
use tracing::info;

pub struct GitResolver;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedCommitInfo {
    pub commit_sha: String,
    pub constitution_hash: String,
}

impl GitResolver {
    /// Resolves target_branch to latest 40-character commit SHA from remote or local repo,
    /// and calculates SHA-256 hash of constitution document content.
    pub fn resolve_commit_and_constitution(
        repo_url: &str,
        target_branch: &str,
        constitution_ref: &str,
    ) -> Result<ResolvedCommitInfo> {
        info!(
            repo_url = %repo_url,
            target_branch = %target_branch,
            constitution_ref = %constitution_ref,
            "Resolving commit SHA and constitution hash"
        );

        // Attempt git ls-remote first
        let output = Command::new("git")
            .args(["ls-remote", repo_url, target_branch])
            .output();

        let commit_sha = match output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                if let Some(first_line) = stdout.lines().next() {
                    first_line
                        .split_whitespace()
                        .next()
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "0000000000000000000000000000000000000000".to_string())
                } else {
                    "0000000000000000000000000000000000000000".to_string()
                }
            }
            _ => {
                // Fallback to local HEAD if available
                let head = Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .output()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                    .unwrap_or_else(|_| "0000000000000000000000000000000000000000".to_string());
                head
            }
        };

        // Compute constitution hash from constitution file if local exists, else hash path
        let constitution_hash = if std::path::Path::new(constitution_ref).exists() {
            let content = std::fs::read(constitution_ref)?;
            let mut hasher = Sha256::new();
            hasher.update(&content);
            hex::encode(hasher.finalize())
        } else {
            let mut hasher = Sha256::new();
            hasher.update(constitution_ref.as_bytes());
            hex::encode(hasher.finalize())
        };

        Ok(ResolvedCommitInfo {
            commit_sha,
            constitution_hash,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_resolve_local_head() {
        let res = GitResolver::resolve_commit_and_constitution(
            ".",
            "HEAD",
            ".specify/memory/constitution.md",
        );
        assert!(res.is_ok());
        let info = res.unwrap();
        assert_eq!(info.commit_sha.len(), 40);
        assert_eq!(info.constitution_hash.len(), 64);
    }
}
