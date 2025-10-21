pub mod config;

use crate::config::Config;
use log::error;
use std::{
    env::{self},
    fs::{self, ReadDir},
    iter::Enumerate,
};

pub fn remove_repo(repo_name: String) {
    println!("Removing repo: {}", repo_name);
}

pub fn add_repo() {
    let current_dir_pathbuf = env::current_dir().unwrap();

    let dir_name = current_dir_pathbuf.file_name();
    if dir_name.is_none() {
        error!("Failed to get current directory name");
        return;
    }

    let current_configs = get_all_repos();
    if current_configs.is_none() {
        error!("The configs directory contains nothing");
        return;
    }

    let found_config = current_configs
        .unwrap()
        .find(|x| x.1.as_ref().unwrap().file_name() == dir_name.unwrap());

    if found_config.is_some() {
        error!(
            "{:?} is already configured. If you want to edit the configuration file, you can find it at HOME/zlorbrs/configs/{:?}",
            dir_name, dir_name
        );
        return;
    }

    let _ = Config::load(String::from(dir_name.unwrap().to_str().unwrap()));
}

pub fn list_repos() {
    let repos = get_all_repos();
    if repos.is_none() {
        error!("No configurations found");
        return;
    }

    let mapped_repos = repos.unwrap().map(|item| item.1.unwrap().path());
    println!("{:#?}", Vec::from_iter(mapped_repos));
}

pub fn get_all_repos() -> Option<Enumerate<ReadDir>> {
    let home_dir = match std::env::home_dir() {
        Some(x) => String::from(x.to_str().unwrap()),
        None => {
            error!("Failed to get the home directory");
            panic!("Program exited due to previous error");
        }
    };

    let config_dir = format!("{}/.config/zlorbrs/configs", home_dir);

    if let Ok(dir) = fs::read_dir(config_dir.clone()) {
        return Some(dir.enumerate());
    }

    error!("Config directory doesnt exist. Creating it now...");
    let create_dir_results = fs::create_dir_all(config_dir.clone());
    if let Ok(_) = create_dir_results {
        let files = fs::read_dir(config_dir);
        if files.is_err() {
            error!(
                "Failed to create config directory for reason: {}",
                files.err().unwrap()
            );
            panic!("Exiting due to previous error")
        }
        let files_unwrapped = files.unwrap();
        return Some(files_unwrapped.enumerate());
    }

    error!(
        "Failed to create config directory for reason: {}",
        create_dir_results.err().unwrap()
    );
    panic!("Exiting due to previous error")
}

pub fn start_daemon() {
    println!("Starting a daemon");
}
