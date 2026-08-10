use crate::plugin::types::{ExecutionContext, ExecutionOutput, Plugin, PluginError};
use async_trait::async_trait;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::collections::HashMap;
use std::fs::File;

pub struct ArtifactCachePlugin;

#[async_trait]
impl Plugin for ArtifactCachePlugin {
    async fn execute(
        &self,
        ctx: &ExecutionContext,
        args: &Option<HashMap<String, String>>,
        _log_sender: Option<tokio::sync::mpsc::Sender<String>>,
    ) -> Result<ExecutionOutput, PluginError> {
        let args = args.as_ref().ok_or_else(|| {
            PluginError::ExecutionFailed(
                "ArtifactCachePlugin requires 'path' and 'out' arguments".to_string(),
            )
        })?;

        let src = args.get("path").ok_or_else(|| {
            PluginError::ExecutionFailed("ArtifactCachePlugin requires 'path' argument".to_string())
        })?;

        let out = args.get("out").ok_or_else(|| {
            PluginError::ExecutionFailed("ArtifactCachePlugin requires 'out' argument".to_string())
        })?;

        let src_path = ctx.workspace_path.join(src);
        let out_path = ctx.workspace_path.join(out);

        let src_clone = src_path.clone();
        let out_clone = out_path.clone();

        let res = tokio::task::spawn_blocking(move || -> Result<(), std::io::Error> {
            let tar_gz = File::create(&out_clone)?;
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = tar::Builder::new(enc);
            tar.append_dir_all(".", &src_clone)?;
            Ok(())
        })
        .await
        .map_err(|e| PluginError::ExecutionFailed(e.to_string()))?;

        match res {
            Ok(_) => Ok(ExecutionOutput {
                status: 0,
                stdout: format!("Archived {:?} into {:?}", src_path, out_path),
                stderr: String::new(),
            }),
            Err(e) => Err(PluginError::ExecutionFailed(format!(
                "Tar archive failed: {}",
                e
            ))),
        }
    }
}
