mod models;

use crate::models::{
    ClusterStatus, LogEvent, NodeStatusEvent, PipelineInfo, PipelineResponse, PipelineTopology,
    PipelineTopologyEdge, PipelineTopologyNode,
};
use nautilus_core::engine::k8s::KubeClient;
use nautilus_core::engine::scheduler::PipelineRunner;
use nautilus_core::model::pipeline::Pipeline;
use std::fs;
use tauri::{AppHandle, Emitter};
// use std::path::PathBuf;
use tokio::sync::mpsc;

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
async fn list_pipelines() -> Result<Vec<PipelineInfo>, String> {
    let mut pipelines = Vec::new();
    let entries = fs::read_dir(".").map_err(|e| e.to_string())?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "yml" || ext == "yaml" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(pipeline) = Pipeline::from_yaml(&content) {
                            pipelines.push(PipelineInfo {
                                path: path.to_string_lossy().to_string(),
                                name: pipeline.name.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
    Ok(pipelines)
}

#[tauri::command]
async fn load_pipeline(path: String) -> Result<PipelineTopology, String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let pipeline = Pipeline::from_yaml(&content).map_err(|e| format!("{:?}", e))?;

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for stage in &pipeline.stages {
        for task in &stage.tasks {
            nodes.push(PipelineTopologyNode {
                id: task.id.clone(),
                label: task.name.clone().unwrap_or(task.id.clone()),
            });

            if let Some(deps) = &task.depends_on {
                for dep in deps {
                    edges.push(PipelineTopologyEdge {
                        source: dep.clone(),
                        target: task.id.clone(),
                    });
                }
            }
        }
    }

    Ok(PipelineTopology { nodes, edges })
}

#[tauri::command]
async fn run_pipeline(
    app: AppHandle,
    manifest_path: Option<String>,
) -> Result<PipelineResponse, String> {
    let path = manifest_path.unwrap_or_else(|| "pipelines.yml".to_string());
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let pipeline = Pipeline::from_yaml(&content).map_err(|e| format!("{:?}", e))?;

    let (tx, mut rx) = mpsc::channel::<String>(100);

    tauri::async_runtime::spawn(async move {
        // Stream logs to UI
        let app_clone = app.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                // Parse msg to see if it's a node status event
                if msg.contains("STARTED") || msg.contains("SUCCESS") || msg.contains("FAILED") {
                    if let Some(start) = msg.find('[') {
                        if let Some(end) = msg.find(']') {
                            let node_id = &msg[start + 1..end];
                            let status = if msg.contains("STARTED") {
                                "running"
                            } else if msg.contains("SUCCESS") {
                                "success"
                            } else {
                                "failed"
                            };
                            let _ = app_clone.emit(
                                "node-status",
                                NodeStatusEvent {
                                    node_id: node_id.to_string(),
                                    status: status.to_string(),
                                },
                            );
                        }
                    }
                }

                let _ = app_clone.emit("pipeline-log", LogEvent { line: msg });
            }
            let _ = app_clone.emit(
                "pipeline-log",
                LogEvent {
                    line: "Pipeline execution finished.".to_string(),
                },
            );
        });

        let runner = PipelineRunner::new(4, std::env::current_dir().unwrap());
        let _ = runner.run(&pipeline, Some(tx)).await;
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
        .invoke_handler(tauri::generate_handler![
            get_cluster_status,
            list_pipelines,
            load_pipeline,
            run_pipeline
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
