pub mod builtins;
pub mod types;

pub use types::{ExecutionContext, ExecutionOutput, Plugin, PluginError};

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::HashMap;

    struct MockPlugin;

    #[async_trait]
    impl Plugin for MockPlugin {
        async fn execute(
            &self,
            _ctx: &ExecutionContext,
            _args: &Option<HashMap<String, String>>,
            _log_sender: Option<tokio::sync::mpsc::Sender<String>>,
        ) -> Result<ExecutionOutput, PluginError> {
            Ok(ExecutionOutput {
                status: 0,
                stdout: "mocked".to_string(),
                stderr: "".to_string(),
            })
        }
    }

    #[tokio::test]
    async fn test_mock_plugin() {
        let plugin = MockPlugin;
        let ctx = ExecutionContext {
            env: HashMap::new(),
            workspace_path: std::path::PathBuf::from("/tmp"),
        };

        let output = plugin.execute(&ctx, &None, None).await.unwrap();
        assert_eq!(output.status, 0);
        assert_eq!(output.stdout, "mocked");
    }
}
