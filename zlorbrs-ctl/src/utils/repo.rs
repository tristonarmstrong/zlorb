use log::{error, info};
use std::{
    env,
    fs::{self, ReadDir},
    iter::Enumerate,
};
use zlorbrs_lib::{config::Config, get_home_dir};

/// .
///
/// # Removes a repo
///
/// Looks in the repo config directory and if `repo_name` is
/// found, will attempt to delete  the repo config
pub(crate) fn remove(repo_name: String) {
    let repos = self::get_all();
    if repos.is_none() {
        error!("There are no repos to remove");
        return;
    }
    let repos = repos.unwrap();
    let mut mapped_repos = repos.map(|item| {
        let x = item.1;
        if x.is_err() {
            error!("Failed to map repos to good type.. aborting");
            panic!("Panic due to previous error");
        }
        let x = x.unwrap();
        x.path()
    });

    let found = mapped_repos.find(|item| {
        let file_name = item.file_name();
        if file_name.is_none() {
            return false;
        }
        let file_name = file_name.unwrap();
        let file_name = file_name.to_str();
        if file_name.is_none() {
            return false;
        }
        let file_name = file_name.unwrap();
        return file_name == &repo_name;
    });

    if found.is_none() {
        error!("Theres no config found with name: {}", repo_name);
        return;
    }
    let found = found.unwrap();
    match std::fs::remove_dir_all(found) {
        Ok(_) => {
            info!("Removed config for: {}", repo_name);
        }
        Err(e) => {
            error!("Unable to remove config for {} because: {:?}", repo_name, e);
        }
    };
}

/// .
///
/// # Panics
///
/// Panics if .
pub(crate) fn get_all() -> Option<Enumerate<ReadDir>> {
    let home_dir = get_home_dir();

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

/// .
///
/// # Panics
///
/// Panics if .
pub(crate) fn add() {
    let current_dir_pathbuf = env::current_dir().unwrap();

    let dir_name = current_dir_pathbuf.file_name();
    if dir_name.is_none() {
        error!("Failed to get current directory name");
        return;
    }

    let current_configs = self::get_all();
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

/// .
///
/// # Panics
///
/// Panics if .
pub(crate) fn list() {
    let repos = self::get_all();
    if repos.is_none() {
        error!("No configurations found");
        return;
    }

    let mapped_repos = repos.unwrap().map(|item| item.1.unwrap().path());
    println!("{:#?}", Vec::from_iter(mapped_repos));
}
