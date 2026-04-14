mod utils;
use crate::utils::{daemon, repo};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Remove {
        #[arg(short, long)]
        repo_name: String,
    },
    Add {
        #[arg(long, default_value = "bun")]
        project_type: String,
        #[arg(long)]
        run_command: Option<String>,
    },
    List,
    Start,
}

fn main() {
    // when running the program, use RUST_LOG with (error, info, debug)
    env_logger::init();

    let args = Args::parse();

    match args.cmd {
        Commands::Add { project_type, run_command } => repo::add(project_type, run_command),
        Commands::List => repo::list(),
        Commands::Start => daemon::start(),
        Commands::Remove { repo_name } => repo::remove(repo_name),
    }
}
