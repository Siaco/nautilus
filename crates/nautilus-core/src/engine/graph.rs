use crate::model::error::PipelineParseError;
use crate::model::pipeline::Pipeline;
use petgraph::algo::{is_cyclic_directed, toposort};
use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PipelineGraph<'a> {
    pub graph: DiGraphMap<&'a str, ()>,
    pub task_map: HashMap<&'a str, &'a crate::model::pipeline::Task>,
}

impl<'a> PipelineGraph<'a> {
    pub fn new(pipeline: &'a Pipeline) -> Result<Self, PipelineParseError> {
        let mut graph = DiGraphMap::new();
        let mut task_map = HashMap::new();

        for stage in &pipeline.stages {
            for task in &stage.tasks {
                graph.add_node(task.id.as_str());
                task_map.insert(task.id.as_str(), task);
            }
        }

        for stage in &pipeline.stages {
            for task in &stage.tasks {
                if let Some(deps) = &task.depends_on {
                    for dep in deps {
                        if !graph.contains_node(dep.as_str()) {
                            return Err(PipelineParseError::ValidationError(format!(
                                "Task '{}' depends on unknown task '{}'",
                                task.id, dep
                            )));
                        }
                        graph.add_edge(dep.as_str(), task.id.as_str(), ());
                    }
                }
            }
        }

        if is_cyclic_directed(&graph) {
            return Err(PipelineParseError::ValidationError(
                "Circular dependency detected in pipeline".to_string(),
            ));
        }

        Ok(Self { graph, task_map })
    }

    pub fn topological_sort(&self) -> Result<Vec<&'a str>, PipelineParseError> {
        match toposort(&self.graph, None) {
            Ok(sorted) => Ok(sorted),
            Err(_) => Err(PipelineParseError::ValidationError(
                "Circular dependency detected in pipeline".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::pipeline::{Pipeline, Stage, Task};

    #[test]
    fn test_valid_dag() {
        let pipeline = Pipeline {
            name: "test".to_string(),
            description: None,
            stages: vec![Stage {
                id: "build".to_string(),
                name: None,
                tasks: vec![
                    Task {
                        id: "task_a".to_string(),
                        name: None,
                        depends_on: None,
                        steps: vec![],
                    },
                    Task {
                        id: "task_b".to_string(),
                        name: None,
                        depends_on: Some(vec!["task_a".to_string()]),
                        steps: vec![],
                    },
                ],
            }],
        };

        let graph = PipelineGraph::new(&pipeline).unwrap();
        let sorted = graph.topological_sort().unwrap();
        assert_eq!(sorted, vec!["task_a", "task_b"]);
    }

    #[test]
    fn test_circular_dependency() {
        let pipeline = Pipeline {
            name: "test".to_string(),
            description: None,
            stages: vec![Stage {
                id: "build".to_string(),
                name: None,
                tasks: vec![
                    Task {
                        id: "task_a".to_string(),
                        name: None,
                        depends_on: Some(vec!["task_b".to_string()]),
                        steps: vec![],
                    },
                    Task {
                        id: "task_b".to_string(),
                        name: None,
                        depends_on: Some(vec!["task_a".to_string()]),
                        steps: vec![],
                    },
                ],
            }],
        };

        let err = PipelineGraph::new(&pipeline).unwrap_err();
        match err {
            PipelineParseError::ValidationError(msg) => {
                assert!(msg.contains("Circular dependency"))
            }
            _ => panic!("Expected ValidationError"),
        }
    }
}
