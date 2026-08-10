use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::fs;
use anyhow::Result;

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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { file } => {
            // Check if the file exists
            if !file.exists() {
                // If it's the default "pipelines.yml", create a boilerplate
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

            println!("Executing pipeline from {:?}", file);
            // We will launch the TUI here in the next task.
        }
        Commands::Studio => {
            println!("Launching Nautilus Studio...");
            // TODO: Launch the desktop GUI
        }
    }

    Ok(())
}
