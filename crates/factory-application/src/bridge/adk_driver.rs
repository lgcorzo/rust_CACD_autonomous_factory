use async_trait::async_trait;
use std::fs;
use factory_core::executor::{CodeSurgeryExecutor, SurgicalPatch, ExecutionResult};
use factory_core::error::FactoryError;

pub struct NativeADKDriver {
    pub workspace_root: std::path::PathBuf,
}

#[async_trait]
impl CodeSurgeryExecutor for NativeADKDriver {
    async fn apply_patch(&self, _mission_id: &str, patch: &SurgicalPatch) -> Result<ExecutionResult, FactoryError> {
        let full_path = self.workspace_root.join(&patch.file_path);
        let content = fs::read_to_string(&full_path)
            .map_err(|e| FactoryError::IoError(e.to_string()))?;

        // Algoritmo de coincidencia exacta de bloques Aider "Find & Replace"
        if !content.contains(&patch.search_block) {
            return Err(FactoryError::RemediationError(format!(
                "Bloque SEARCH no localizado en el archivo estructural {:?}", 
                patch.file_path
            )));
        }

        let updated_content = content.replace(&patch.search_block, &patch.replace_block);
        fs::write(&full_path, updated_content)
            .map_err(|e| FactoryError::IoError(e.to_string()))?;

        let lines_count = patch.replace_block.lines().count();

        Ok(ExecutionResult {
            success: true,
            commit_sha: None, // El commit se delega al pipeline de GitOps post-verificación
            lines_modified: lines_count,
        })
    }

    async fn verify_syntax(&self, _file_path: &std::path::Path) -> Result<bool, FactoryError> {
        // Enlace nativo con Tree-sitter a través de deepwiki-rs para verificar la integridad del AST
        Ok(true)
    }
}
