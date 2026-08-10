use crate::plugin::types::{ExecutionContext, ExecutionOutput, Plugin, PluginError};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use std::process::Stdio;

pub struct ShellExecPlugin;

#[async_trait]
impl Plugin for ShellExecPlugin {
    async fn execute(
        &self,
        ctx: &ExecutionContext,
        args: &Option<HashMap<String, String>>,
        log_sender: Option<tokio::sync::mpsc::Sender<String>>,
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
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        for (k, v) in &ctx.env {
            cmd.env(k, v);
        }

        let mut child = cmd.spawn().map_err(|e| {
            PluginError::ExecutionFailed(format!("Failed to spawn shell command: {}", e))
        })?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let sender_out = log_sender.clone();
        let stdout_handle = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            let mut output = String::new();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Some(s) = &sender_out {
                    let _ = s.send(line.clone()).await;
                }
                output.push_str(&line);
                output.push('\n');
            }
            output
        });

        let sender_err = log_sender.clone();
        let stderr_handle = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            let mut output = String::new();
            while let Ok(Some(line)) = reader.next_line().await {
                if let Some(s) = &sender_err {
                    let _ = s.send(format!("ERROR: {}", line)).await;
                }
                output.push_str(&line);
                output.push('\n');
            }
            output
        });

        let status = child.wait().await.map_err(|e| {
            PluginError::ExecutionFailed(format!("Failed to wait on child process: {}", e))
        })?;

        let stdout_str = stdout_handle.await.unwrap_or_default();
        let stderr_str = stderr_handle.await.unwrap_or_default();

        Ok(ExecutionOutput {
            status: status.code().unwrap_or(1),
            stdout: stdout_str,
            stderr: stderr_str,
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
        let output = plugin.execute(&ctx, &Some(args), None).await.unwrap();

        assert_eq!(output.status, 0);
        assert!(output.stdout.contains("hello"));
    }
}
