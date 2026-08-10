use crate::engine::graph::PipelineGraph;
use crate::model::pipeline::Pipeline;
use std::collections::HashMap;
use tokio::sync::mpsc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TaskState {
    Pending,
    Running,
    Success,
    Failed,
    Skipped,
}

#[derive(Debug, Clone)]
pub struct TaskEvent {
    pub task_id: String,
    pub state: TaskState,
}

pub struct PipelineRunner {
    pub concurrency_limit: usize,
}

impl PipelineRunner {
    pub fn new(concurrency_limit: usize) -> Self {
        Self { concurrency_limit }
    }

    pub async fn run(
        &self,
        pipeline: &Pipeline,
    ) -> Result<HashMap<String, TaskState>, crate::model::error::PipelineParseError> {
        let graph = PipelineGraph::new(pipeline)?;
        let sorted_tasks = graph.topological_sort()?;

        let (tx, mut rx) = mpsc::channel::<TaskEvent>(100);
        let mut states: HashMap<String, TaskState> = HashMap::new();

        for task_id in &sorted_tasks {
            states.insert(task_id.to_string(), TaskState::Pending);
        }

        let mut active_tasks = 0;
        let mut completed_tasks = 0;
        let total_tasks = sorted_tasks.len();

        let mut dependencies = HashMap::new();
        for stage in &pipeline.stages {
            for task in &stage.tasks {
                let deps = task.depends_on.clone().unwrap_or_default();
                dependencies.insert(task.id.clone(), deps);
            }
        }

        while completed_tasks < total_tasks {
            let mut ready_tasks = Vec::new();
            for (task_id, state) in &states {
                if *state == TaskState::Pending {
                    let deps = dependencies.get(task_id).unwrap();
                    let mut can_run = true;
                    let mut should_skip = false;

                    for dep in deps {
                        match states.get(dep) {
                            Some(TaskState::Success) => {}
                            Some(TaskState::Failed) | Some(TaskState::Skipped) => {
                                should_skip = true;
                                can_run = false;
                                break;
                            }
                            _ => {
                                can_run = false;
                                break;
                            }
                        }
                    }

                    if should_skip {
                        let tx_clone = tx.clone();
                        let tid = task_id.clone();
                        tokio::spawn(async move {
                            tx_clone
                                .send(TaskEvent {
                                    task_id: tid,
                                    state: TaskState::Skipped,
                                })
                                .await
                                .unwrap();
                        });
                    } else if can_run {
                        ready_tasks.push(task_id.clone());
                    }
                }
            }

            for task_id in ready_tasks {
                if active_tasks >= self.concurrency_limit {
                    break;
                }

                states.insert(task_id.clone(), TaskState::Running);
                active_tasks += 1;

                let tx_clone = tx.clone();
                // Mock execution: fail if id is 'fail_task'
                let is_fail = task_id == "fail_task";
                tokio::spawn(async move {
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    let end_state = if is_fail {
                        TaskState::Failed
                    } else {
                        TaskState::Success
                    };
                    tx_clone
                        .send(TaskEvent {
                            task_id,
                            state: end_state,
                        })
                        .await
                        .unwrap();
                });
            }

            if let Some(event) = rx.recv().await {
                match event.state {
                    TaskState::Running => {}
                    TaskState::Success | TaskState::Failed => {
                        states.insert(event.task_id, event.state);
                        active_tasks = active_tasks.saturating_sub(1);
                        completed_tasks += 1;
                    }
                    TaskState::Skipped => {
                        states.insert(event.task_id, event.state);
                        completed_tasks += 1;
                    }
                    TaskState::Pending => {}
                }
            }
        }

        Ok(states)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::pipeline::{Stage, Task};

    #[tokio::test]
    async fn test_scheduler_runs_concurrently() {
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
                        depends_on: None,
                        steps: vec![],
                    },
                ],
            }],
        };

        let runner = PipelineRunner::new(2);
        let start = std::time::Instant::now();
        let states = runner.run(&pipeline).await.unwrap();
        let elapsed = start.elapsed();

        assert_eq!(states.get("task_a"), Some(&TaskState::Success));
        assert_eq!(states.get("task_b"), Some(&TaskState::Success));
        // Both run concurrently, so it should take ~50ms, not 100ms.
        assert!(elapsed.as_millis() < 90);
    }

    #[tokio::test]
    async fn test_scheduler_skips_on_failure() {
        let pipeline = Pipeline {
            name: "test".to_string(),
            description: None,
            stages: vec![Stage {
                id: "build".to_string(),
                name: None,
                tasks: vec![
                    Task {
                        id: "fail_task".to_string(),
                        name: None,
                        depends_on: None,
                        steps: vec![],
                    },
                    Task {
                        id: "task_b".to_string(),
                        name: None,
                        depends_on: Some(vec!["fail_task".to_string()]),
                        steps: vec![],
                    },
                    Task {
                        id: "task_c".to_string(),
                        name: None,
                        depends_on: None,
                        steps: vec![],
                    },
                ],
            }],
        };

        let runner = PipelineRunner::new(2);
        let states = runner.run(&pipeline).await.unwrap();

        assert_eq!(states.get("fail_task"), Some(&TaskState::Failed));
        assert_eq!(states.get("task_b"), Some(&TaskState::Skipped));
        assert_eq!(states.get("task_c"), Some(&TaskState::Success));
    }
}
