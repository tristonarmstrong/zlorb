use std::{fs, io};

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,          // repo identifier
    pub path: String,          // absolute path to repo
    pub branch: String,        // e.g. main
    pub remote: String,        // e.g. origin
    pub build_command: String, // e.g. npm run build
}

impl Config {
    pub fn new(repo_name: String) -> Self {
        Self {
            name: repo_name,
            path: String::from(std::env::current_dir().unwrap().to_str().unwrap()),
            branch: String::from("master"),
            remote: String::from("origin"),
            build_command: String::from("bun run build"),
        }
    }

    pub fn load(repo_name: String) -> Result<String, io::Error> {
        info!("Loading config for {}", repo_name);
        let mut contents = fs::read_to_string(format!(
            "{}/.config/zlorbrs/configs/{}",
            std::env::home_dir().unwrap().to_str().unwrap(),
            repo_name
        ));
        if contents.is_err() {
            info!("Theres no config so we need to create one");
            contents = Ok(Self::save(repo_name));
        }
        info!("Found contents: {:#?}", contents);
        contents
    }

    pub fn save(repo_name: String) -> String {
        info!("Generating configuration file. System assumes Bun build script");
        let directory_path = format!(
            "{}/.config/zlorbrs/configs/{}",
            std::env::home_dir().unwrap().to_str().unwrap(),
            repo_name
        );
        let file_path = format!("{directory_path}/config.json");

        // first create directory
        match std::fs::create_dir_all(directory_path.clone()) {
            Ok(_) => {
                println!("Created config directory at: {directory_path}")
            }
            Err(e) => panic!("{e}"),
        };

        // then write file
        let data = serde_json::to_string(&Config::new(repo_name)).unwrap();
        let raw = data.clone();
        match std::fs::write(file_path.clone(), raw) {
            Ok(_) => {
                println!("Created configuration file at: {file_path}")
            }
            Err(e) => panic!("{e}"),
        };
        data
    }
}
