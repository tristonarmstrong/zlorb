use std::fs::ReadDir;

use clap::{Parser, Subcommand};
use zlorbrs_lib::{add_repo, get_all_repos, list_repos, remove_repo, start_daemon};

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
    Add,
    List,
    Start,
}

fn main() {
    // when running the program, use RUST_LOG with (error, info, debug)
    env_logger::init();

    let args = Args::parse();

    match args.cmd {
        Commands::Add => add_repo(),
        Commands::List => list_repos(),
        Commands::Start => start_daemon(),
        Commands::Remove { repo_name } => remove_repo(repo_name),
    }
}
