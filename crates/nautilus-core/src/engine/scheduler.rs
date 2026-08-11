use crate::engine::graph::PipelineGraph;
use crate::model::pipeline::{Pipeline, Task};
use crate::plugin::builtins::shell::ShellExecPlugin;
use crate::plugin::types::{ExecutionContext, Plugin};
use std::collections::HashMap;
use std::path::PathBuf;
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
    pub workspace_path: PathBuf,
}

impl PipelineRunner {
    pub fn new(concurrency_limit: usize, workspace_path: PathBuf) -> Self {
        Self {
            concurrency_limit,
            workspace_path,
        }
    }

    pub async fn run(
        &self,
        pipeline: &Pipeline,
        log_sender: Option<mpsc::Sender<String>>,
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
        let mut task_definitions: HashMap<String, Task> = HashMap::new();

        for stage in &pipeline.stages {
            for task in &stage.tasks {
                let deps = task.depends_on.clone().unwrap_or_default();
                dependencies.insert(task.id.clone(), deps);
                task_definitions.insert(task.id.clone(), task.clone());
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
                let task_def = task_definitions.get(&task_id).unwrap().clone();
                let workspace = self.workspace_path.clone();
                let global_log_sender = log_sender.clone();

                tokio::spawn(async move {
                    let mut success = true;

                    if let Some(sender) = &global_log_sender {
                        let _ = sender.send(format!("[{}] STARTED", task_id)).await;
                    }

                    for step in task_def.steps {
                        let plugin: Box<dyn Plugin + Send + Sync> = match step.plugin.as_str() {
                            "shell" => Box::new(ShellExecPlugin),
                            _ => {
                                if let Some(sender) = &global_log_sender {
                                    let _ = sender
                                        .send(format!(
                                            "[{}] ERROR: Unsupported plugin '{}'",
                                            task_id, step.plugin
                                        ))
                                        .await;
                                }
                                success = false;
                                break;
                            }
                        };

                        let ctx = ExecutionContext {
                            env: HashMap::new(),
                            workspace_path: workspace.clone(),
                        };

                        let step_log_sender = if let Some(sender) = &global_log_sender {
                            let (step_tx, mut step_rx) = mpsc::channel(100);
                            let tid = task_id.clone();
                            let s2 = sender.clone();
                            tokio::spawn(async move {
                                while let Some(msg) = step_rx.recv().await {
                                    let _ = s2.send(format!("[{}] {}", tid, msg)).await;
                                }
                            });
                            Some(step_tx)
                        } else {
                            None
                        };

                        let result = plugin.execute(&ctx, &step.with, step_log_sender).await;
                        match result {
                            Ok(out) if out.status == 0 => {
                                // step success
                            }
                            _ => {
                                success = false;
                                break;
                            }
                        }
                    }

                    if let Some(sender) = &global_log_sender {
                        let msg = if success {
                            format!("[{}] SUCCESS", task_id)
                        } else {
                            format!("[{}] FAILED", task_id)
                        };
                        let _ = sender.send(msg).await;
                    }

                    let end_state = if success {
                        TaskState::Success
                    } else {
                        TaskState::Failed
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
                        steps: vec![crate::model::pipeline::Step {
                            id: "s1".to_string(),
                            plugin: "unknown_fails".to_string(),
                            with: None,
                        }],
                    },
                    Task {
                        id: "task_b".to_string(),
                        name: None,
                        depends_on: Some(vec!["fail_task".to_string()]),
                        steps: vec![],
                    },
                ],
            }],
        };

        let runner = PipelineRunner::new(2, PathBuf::from("."));
        let states = runner.run(&pipeline, None).await.unwrap();

        assert_eq!(states.get("fail_task"), Some(&TaskState::Failed));
        assert_eq!(states.get("task_b"), Some(&TaskState::Skipped));
    }
}
