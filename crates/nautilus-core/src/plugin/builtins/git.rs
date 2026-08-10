use crate::plugin::types::{ExecutionContext, ExecutionOutput, Plugin, PluginError};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::process::Command;

pub struct GitCheckoutPlugin;

#[async_trait]
impl Plugin for GitCheckoutPlugin {
    async fn execute(
        &self,
        ctx: &ExecutionContext,
        args: &Option<HashMap<String, String>>,
        _log_sender: Option<tokio::sync::mpsc::Sender<String>>,
    ) -> Result<ExecutionOutput, PluginError> {
        let args = args.as_ref().ok_or_else(|| {
            PluginError::ExecutionFailed("GitCheckoutPlugin requires 'url' argument".to_string())
        })?;

        let url = args.get("url").ok_or_else(|| {
            PluginError::ExecutionFailed("GitCheckoutPlugin requires 'url' argument".to_string())
        })?;

        let path = &ctx.workspace_path;

        let mut cmd = Command::new("git");
        cmd.arg("clone").arg(url).arg(path);

        let output = cmd.output().await.map_err(|e| {
            PluginError::ExecutionFailed(format!("Failed to execute git clone: {}", e))
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status = output.status.code().unwrap_or(1);

        if status != 0 {
            return Err(PluginError::ExecutionFailed(format!(
                "Git clone failed: {}",
                stderr
            )));
        }

        Ok(ExecutionOutput {
            status,
            stdout,
            stderr,
        })
    }
}
