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

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/PipelineInfo.ts")]
pub struct PipelineInfo {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/PipelineTopologyNode.ts")]
pub struct PipelineTopologyNode {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/PipelineTopologyEdge.ts")]
pub struct PipelineTopologyEdge {
    pub source: String,
    pub target: String,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/PipelineTopology.ts")]
pub struct PipelineTopology {
    pub nodes: Vec<PipelineTopologyNode>,
    pub edges: Vec<PipelineTopologyEdge>,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "../src/bindings/NodeStatusEvent.ts")]
pub struct NodeStatusEvent {
    pub node_id: String,
    pub status: String, // "pending", "running", "success", "failed"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_bindings() {
        ClusterStatus::export().unwrap();
        PipelineResponse::export().unwrap();
        LogEvent::export().unwrap();
        PipelineInfo::export().unwrap();
        PipelineTopologyNode::export().unwrap();
        PipelineTopologyEdge::export().unwrap();
        PipelineTopology::export().unwrap();
        NodeStatusEvent::export().unwrap();
    }
}
