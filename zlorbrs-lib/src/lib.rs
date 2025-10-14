pub mod config;

use crate::config::Config;
use log::error;
use std::{
    env,
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

    Config::load(String::from(dir_name.unwrap().to_str().unwrap()));
}

pub fn list_repos() {
    let repos: Vec<_> = get_all_repos().unwrap().collect();
    let repo_count = repos.len();
    if repo_count < 1 as usize {
        println!("No configurations found");
        return;
    }
    repos.iter().for_each(|x| println!("{:?}", x.1));
}

pub fn get_all_repos() -> Option<Enumerate<ReadDir>> {
    let config_dir = format!(
        "{}/.config/zlorbrs/configs",
        std::env::home_dir().unwrap().to_str().unwrap()
    );
    match fs::read_dir(config_dir.clone()) {
        Ok(dir) => Some(dir.enumerate()),
        Err(_) => {
            error!("Config directory doesnt exist. Creating it now...");
            match fs::create_dir_all(config_dir.clone()) {
                Ok(_) => Some(fs::read_dir(config_dir).unwrap().enumerate()),
                Err(err) => {
                    error!("Failed to create config directory for reason: {}", err);
                    panic!("Exiting due to previous error")
                }
            }
        }
    }
}

pub fn start_daemon() {
    println!("Starting a daemon");
}
