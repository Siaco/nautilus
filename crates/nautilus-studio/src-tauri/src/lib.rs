mod models;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use crate::models::{ClusterStatus, LogEvent, PipelineResponse};
use nautilus_core::engine::k8s::KubeClient;
use tauri::{AppHandle, Emitter};

#[tauri::command]
async fn get_cluster_status() -> Result<ClusterStatus, String> {
    let client_res = KubeClient::new().await;
    match client_res {
        Ok(_) => Ok(ClusterStatus {
            is_connected: true,
            node_count: 1, // Mock value
            pod_count: 5,  // Mock value
            version: "v1.27.0".to_string(),
        }),
        Err(e) => Err(format!("Failed to connect to Kubernetes: {}", e)),
    }
}

#[tauri::command]
async fn run_pipeline(
    app: AppHandle,
    _manifest_path: Option<String>,
) -> Result<PipelineResponse, String> {
    tauri::async_runtime::spawn(async move {
        for i in 1..=10 {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            let _ = app.emit(
                "pipeline-log",
                LogEvent {
                    line: format!("Executing step {}...", i),
                },
            );
        }
        let _ = app.emit(
            "pipeline-log",
            LogEvent {
                line: "Pipeline execution completed successfully.".to_string(),
            },
        );
    });

    Ok(PipelineResponse {
        success: true,
        message: "Pipeline started".to_string(),
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_cluster_status, run_pipeline])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
