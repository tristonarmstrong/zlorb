use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    name: String,          // repo identifier
    path: String,          // absolute path to repo
    branch: String,        // e.g. main
    remote: String,        // e.g. origin
    build_command: String, // e.g. npm run build
}

impl Config {
    pub fn load(repo_name: String) {
        let contents =
            fs::read_to_string(format!("/etc/zlorbrs/configs/{}", repo_name)).unwrap_or_default();
    }

    fn save(repo_name: String) {
        unimplemented!();
    }
}
