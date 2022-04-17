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
    Init { x: Option<String>},
    /// Runs the provided npm script
    Run { command: Option<String> },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { x: _ } => {
            // setup a intaractive script
            // to get users info and save it to a config
            todo!("Haven't got this going yet.")
        }
        Commands::Run { command } => {
        Command::new("npm")
            .arg(command.as_ref().unwrap())
            .stdout(Stdio::inherit())
            .output()
            .expect("Failed to execute command");
        }
    }
}
