use crate::error::FactoryError;
use async_trait::async_trait;
use std::path::PathBuf;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SurgicalPatch {
    pub file_path: PathBuf,
    pub search_block: String,
    pub replace_block: String,
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    pub commit_sha: Option<String>,
    pub lines_modified: usize,
}

#[async_trait]
pub trait CodeSurgeryExecutor: Send + Sync {
    async fn apply_patch(
        &self,
        mission_id: &str,
        patch: &SurgicalPatch,
    ) -> Result<ExecutionResult, FactoryError>;
    async fn verify_syntax(&self, file_path: &std::path::Path) -> Result<bool, FactoryError>;
}
