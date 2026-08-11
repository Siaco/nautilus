pub mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use std::fs;
use std::panic;
use std::path::PathBuf;
use ui::app::App;

#[derive(Parser)]
#[command(name = "nautilus")]
#[command(about = "Nautilus Execution Engine CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute a pipeline
    Run {
        /// Path to the pipeline YAML file. Defaults to `pipelines.yml`.
        #[arg(default_value = "pipelines.yml")]
        file: PathBuf,
    },
    /// Launch the Nautilus Studio desktop GUI
    Studio,
}

fn setup_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = std::io::stdout().execute(LeaveAlternateScreen);
        original_hook(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_panic_hook();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => {
            if !file.exists() {
                if file.file_name().and_then(|n| n.to_str()) == Some("pipelines.yml") {
                    println!("pipelines.yml not found. Creating a default template...");
                    let boilerplate = r#"version: "1.0"
pipeline:
  name: "Default Pipeline"
  stages:
    - id: "hello"
      plugin: "shell"
      args:
        command: "echo 'Hello from Nautilus!'"
"#;
                    fs::write(file, boilerplate)?;
                    println!("Created pipelines.yml. Run the command again to execute it.");
                    return Ok(());
                } else {
                    anyhow::bail!("File not found: {:?}", file);
                }
            }

            let (log_sender, log_receiver) = tokio::sync::mpsc::channel(100);

            let content = fs::read_to_string(&file)?;
            match nautilus_core::model::pipeline::Pipeline::from_yaml(&content) {
                Ok(pipeline) => {
                    tokio::spawn(async move {
                        let runner = nautilus_core::engine::scheduler::PipelineRunner::new(
                            4,
                            std::env::current_dir().unwrap(),
                        );
                        let _ = runner.run(&pipeline, Some(log_sender)).await;
                    });
                }
                Err(e) => {
                    anyhow::bail!("Failed to parse pipeline: {:?}", e);
                }
            }

            let mut app = App::new(log_receiver);
            app.run().await?;
        }
        Commands::Studio => {
            println!("Launching Nautilus Studio...");
        }
    }

    Ok(())
}
