use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Runs the provided npm script
    Run { command: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { command } => {

        let output = Command::new("npm")
            .arg(command.as_ref().unwrap())
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to execute command");

            assert_eq!(String::from_utf8_lossy(&output.stdout), "");
        }
    }
}
