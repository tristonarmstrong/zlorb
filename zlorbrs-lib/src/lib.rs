mod config;

use crate::config::Config;
use log::error;
use std::{
    env,
    fs::{self, ReadDir},
    iter::Enumerate,
};

const CONFIG_DIR: &str = "/etc/zlorbrs/configs/";

pub fn remove_repo(repo_name: String) {
    println!("Removing repo: {}", repo_name);
}
pub fn add_repo() {
    let current_dir_pathbuf = env::current_dir().unwrap();
    if let Some(dir_name) = current_dir_pathbuf.file_name() {
        let mut current_configs = get_all_repos();
        if let Some(_) = current_configs
            .unwrap()
            .find(|x| x.1.as_ref().unwrap().file_name() == dir_name)
        {
            error!(
                "{:?} is already configured. If you want to edit the configuration file, you can find it at /etc/zlorbrs/configs/{:?}",
                dir_name, dir_name
            );
            return;
        }
        Config::load(String::from(dir_name.to_str().unwrap()));
    }
}
pub fn get_all_repos() -> Option<Enumerate<ReadDir>> {
    match fs::read_dir(CONFIG_DIR) {
        Ok(dir) => Some(dir.enumerate()),
        Err(_) => {
            error!("Config directory doesnt exist. Creating it now...");
            match fs::create_dir_all(CONFIG_DIR) {
                Ok(_) => Some(fs::read_dir(CONFIG_DIR).unwrap().enumerate()),
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
