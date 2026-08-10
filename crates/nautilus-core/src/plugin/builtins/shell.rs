use crate::plugin::types::{ExecutionContext, ExecutionOutput, Plugin, PluginError};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::process::Command;

pub struct ShellExecPlugin;

#[async_trait]
impl Plugin for ShellExecPlugin {
    async fn execute(
        &self,
        ctx: &ExecutionContext,
        args: &Option<HashMap<String, String>>,
    ) -> Result<ExecutionOutput, PluginError> {
        let args = args.as_ref().ok_or_else(|| {
            PluginError::ExecutionFailed("ShellExecPlugin requires 'run' argument".to_string())
        })?;

        let script = args.get("run").ok_or_else(|| {
            PluginError::ExecutionFailed("ShellExecPlugin requires 'run' argument".to_string())
        })?;

        #[cfg(target_os = "windows")]
        let mut cmd = Command::new("cmd");
        #[cfg(target_os = "windows")]
        cmd.arg("/C").arg(script);

        #[cfg(not(target_os = "windows"))]
        let mut cmd = Command::new("sh");
        #[cfg(not(target_os = "windows"))]
        cmd.arg("-c").arg(script);

        cmd.current_dir(&ctx.workspace_path);

        for (k, v) in &ctx.env {
            cmd.env(k, v);
        }

        let output = cmd.output().await.map_err(|e| {
            PluginError::ExecutionFailed(format!("Failed to execute shell command: {}", e))
        })?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let status = output.status.code().unwrap_or(1);

        Ok(ExecutionOutput {
            status,
            stdout,
            stderr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_shell_exec() {
        let dir = tempdir().unwrap();
        let ctx = ExecutionContext {
            env: HashMap::new(),
            workspace_path: dir.path().to_path_buf(),
        };

        let mut args = HashMap::new();
        args.insert("run".to_string(), "echo hello".to_string());

        let plugin = ShellExecPlugin;
        let output = plugin.execute(&ctx, &Some(args)).await.unwrap();

        assert_eq!(output.status, 0);
        assert!(output.stdout.contains("hello"));
    }
}
