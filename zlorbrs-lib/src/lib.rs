mod config;
use std::env;

// use config::Config;

pub fn remove_repo(repo_name: String) {
    println!("Removing repo: {}", repo_name);
}
pub fn add_repo() {
    println!("Adding repo");
    let current_dir_pathbuf = env::current_dir().unwrap();
    let current_dir_string = current_dir_pathbuf.to_str().unwrap();

    println!("Current directory: {}", current_dir_string);
    // Config::load(current_dir_string);
}
pub fn list_all_repos() {
    println!("List all repos");
}
pub fn start_daemon() {
    println!("Starting a daemon");
}
