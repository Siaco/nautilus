use crate::engine::scheduler::TaskState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowState {
    pub pipeline_name: String,
    pub task_states: HashMap<String, TaskState>,
}

impl WorkflowState {
    pub fn new(pipeline_name: String, task_states: HashMap<String, TaskState>) -> Self {
        Self {
            pipeline_name,
            task_states,
        }
    }

    pub async fn save(&self, path: &Path) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }
        let temp_path = path.with_extension("tmp");
        fs::write(&temp_path, &json).await?;
        fs::rename(&temp_path, path).await?;
        Ok(())
    }

    pub async fn load(path: &Path) -> Result<Self, std::io::Error> {
        let json = fs::read_to_string(path).await?;
        let state: WorkflowState = serde_json::from_str(&json)?;
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_state_serialization() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("state.json");

        let mut states = HashMap::new();
        states.insert("task_a".to_string(), TaskState::Success);
        states.insert("task_b".to_string(), TaskState::Pending);

        let state = WorkflowState::new("test_pipeline".to_string(), states);
        state.save(&file_path).await.unwrap();

        let loaded_state = WorkflowState::load(&file_path).await.unwrap();
        assert_eq!(state, loaded_state);
    }
}
