use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/ClusterStatus.ts")]
pub struct ClusterStatus {
    pub is_connected: bool,
    pub node_count: usize,
    pub pod_count: usize,
    pub version: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/PipelineResponse.ts")]
pub struct PipelineResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/LogEvent.ts")]
pub struct LogEvent {
    pub line: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        ClusterStatus::export().unwrap();
        PipelineResponse::export().unwrap();
        LogEvent::export().unwrap();
    }
}
