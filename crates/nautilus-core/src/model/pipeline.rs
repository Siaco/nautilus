use crate::model::error::PipelineParseError;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pipeline {
    pub name: String,
    pub description: Option<String>,
    pub stages: Vec<Stage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Stage {
    pub id: String,
    pub name: Option<String>,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: String,
    pub name: Option<String>,
    pub depends_on: Option<Vec<String>>,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Step {
    pub id: String,
    pub plugin: String,
    pub with: Option<std::collections::HashMap<String, String>>,
}

impl Pipeline {
    pub fn from_yaml(content: &str) -> Result<Self, PipelineParseError> {
        let pipeline: Pipeline = serde_yaml::from_str(content)?;
        pipeline.validate()?;
        Ok(pipeline)
    }

    pub fn from_toml(content: &str) -> Result<Self, PipelineParseError> {
        let pipeline: Pipeline = toml::from_str(content)?;
        pipeline.validate()?;
        Ok(pipeline)
    }

    pub fn validate(&self) -> Result<(), PipelineParseError> {
        let mut task_ids = HashSet::new();

        for stage in &self.stages {
            for task in &stage.tasks {
                if !task_ids.insert(&task.id) {
                    return Err(PipelineParseError::ValidationError(format!(
                        "Duplicate task ID found: {}",
                        task.id
                    )));
                }
            }
        }

        for stage in &self.stages {
            for task in &stage.tasks {
                if let Some(deps) = &task.depends_on {
                    for dep in deps {
                        if !task_ids.contains(dep) {
                            return Err(PipelineParseError::ValidationError(format!(
                                "Task '{}' depends on unknown task '{}'",
                                task.id, dep
                            )));
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_yaml() {
        let yaml = r#"
name: Test Pipeline
stages:
  - id: build
    tasks:
      - id: build_core
        steps:
          - id: s1
            plugin: shell
  - id: test
    tasks:
      - id: test_core
        depends_on: ["build_core"]
        steps:
          - id: s2
            plugin: shell
"#;
        let pipeline = Pipeline::from_yaml(yaml).unwrap();
        assert_eq!(pipeline.name, "Test Pipeline");
    }

    #[test]
    fn test_invalid_duplicate_id() {
        let yaml = r#"
name: Test Pipeline
stages:
  - id: build
    tasks:
      - id: task1
        steps: []
      - id: task1
        steps: []
"#;
        let err = Pipeline::from_yaml(yaml).unwrap_err();
        match err {
            PipelineParseError::ValidationError(msg) => assert!(msg.contains("Duplicate task ID")),
            _ => panic!("Expected ValidationError"),
        }
    }

    #[test]
    fn test_invalid_dependency() {
        let yaml = r#"
name: Test Pipeline
stages:
  - id: build
    tasks:
      - id: task1
        depends_on: ["unknown_task"]
        steps: []
"#;
        let err = Pipeline::from_yaml(yaml).unwrap_err();
        match err {
            PipelineParseError::ValidationError(msg) => assert!(msg.contains("unknown task")),
            _ => panic!("Expected ValidationError"),
        }
    }
}
