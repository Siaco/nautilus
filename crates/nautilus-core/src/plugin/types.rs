use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub env: HashMap<String, String>,
    pub workspace_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Error, Debug)]
pub enum PluginError {
    #[error("Plugin execution failed: {0}")]
    ExecutionFailed(String),
}

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn execute(
        &self,
        ctx: &ExecutionContext,
        args: &Option<HashMap<String, String>>,
    ) -> Result<ExecutionOutput, PluginError>;
}
