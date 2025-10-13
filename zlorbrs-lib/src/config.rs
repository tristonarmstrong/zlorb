use std::{fs, io};

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    name: String,          // repo identifier
    path: String,          // absolute path to repo
    branch: String,        // e.g. main
    remote: String,        // e.g. origin
    build_command: String, // e.g. npm run build
}

impl Config {
    pub fn new(repo_name: String) -> Self {
        Self {
            name: repo_name,
            path: String::from("TODO"),
            branch: String::from("master"),
            remote: String::from("origin"),
            build_command: String::from("bun run build"),
        }
    }
    pub fn load(repo_name: String) -> Result<String, io::Error> {
        info!("Loading config for {}", repo_name);
        let contents =
            fs::read_to_string(format!("/etc/zlorbrs/configs/{}", repo_name)).unwrap_or_default();
        println!("Found contents: {:#?}", contents);
    }

    fn save(repo_name: String) {
        unimplemented!();
    }
}
