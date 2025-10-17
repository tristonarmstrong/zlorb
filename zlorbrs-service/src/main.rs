use git2::{BranchType, Error, Oid, Remote, Repository};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::{fs, io::Error as IoError};
use zlorbrs_lib::config::Config;

#[derive(Serialize, Deserialize, Default, Debug)]
struct ServiceConfig {
    sleep_time: u64,
}

fn setup_config_stuff() -> Result<ServiceConfig, ()> {
    let path_to_config_file_for_service = format!(
        "{}/.config/zlorbrs/service-config.json",
        std::env::home_dir().unwrap().to_str().unwrap()
    );

    let config_file = match std::fs::read_to_string(path_to_config_file_for_service.clone()) {
        Ok(a) => a,
        Err(e) => {
            error!("{e}");
            error!(
                "You need to ensure you create the config file @ {path_to_config_file_for_service}"
            );
            let config_default: ServiceConfig = Default::default();
            error!(
                "I will at some point do it for you but in the meantime use this: \n{:#?}",
                config_default
            );
            return Err(());
        }
    };

    let config_data = serde_json::from_str::<ServiceConfig>(&config_file)
        .expect("Failed to convert config file to json string");
    Ok(config_data)
}

fn main() -> Result<(), IoError> {
    env_logger::init();

    let config_data = setup_config_stuff().expect("Failed to setup configuration stuff");

    let mut first_run = true;

    loop {
        // Handle napping at first run
        if first_run {
            first_run = false;
        } else {
            take_a_nap(config_data.sleep_time);
        }

        // get config directory and validate its existance
        let dir_path = format!(
            "{}/.config/zlorbrs/configs",
            std::env::home_dir().unwrap().to_str().unwrap()
        );
        let directories = std::fs::read_dir(dir_path);
        if directories.is_err() {
            error!("There are no configuration files created yet");
            continue;
        }
        // </end> get config directory and validate its existance

        // Iterate the items in the configs directory
        directories.unwrap().for_each(|item_wrap| {
            info!(" ");
            // read the configs of each item
            let item = item_wrap.unwrap();
            let file_contents =
                fs::read_to_string(format!("{}/config.json", item.path().to_str().unwrap()))
                    .unwrap();
            let config_json = serde_json::from_str::<Config>(&file_contents).unwrap();
            info!("================ {} ===============", config_json.name);

            let repo = Repository::open(config_json.clone().path).expect("Failed to open repo");

            // check if repo has updates
            let local_branch = repo
                .find_branch(&config_json.branch, BranchType::Local)
                .expect("Local branch not found");
            let local_iod_before: Oid = local_branch
                .get()
                .target()
                .expect("Local branch has no target");
            debug!("before iod: {local_iod_before}");

            // get remote and fetch
            let _ = fast_forward(&repo, &config_json);

            // check if repo has updates
            let local_iod_after: Oid = local_branch
                .get()
                .target()
                .expect("Local branch has no target");
            debug!("after iod: {local_iod_after}");

            if local_iod_before != local_iod_after {
                kick_off_build();
            }
        });
    }
}

fn kick_off_build() {
    std::process::Command::new("ls");
}

fn take_a_nap(sleep_time: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sleep_time));
}

fn fast_forward(repo: &Repository, config_json: &Config) -> Result<(), git2::Error> {
    let remote: Result<Remote, git2::Error> = repo.find_remote("origin");
    if remote.is_err() {
        error!("Remote Not Found");
        return Err(Error::from_str("Remote Not Found"));
    }

    let fetch_res = remote
        .unwrap()
        .fetch(&[config_json.branch.clone()], None, None);
    if fetch_res.is_err() {
        error!("failed to fetch remote: {}", fetch_res.err().unwrap());
    }

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();

    if analysis.0.is_up_to_date() {
        info!("repo is already up to date, skipping fast forward");
        return Ok(());
    }

    if analysis.0.is_fast_forward() {
        info!("Repo needs an update, updating...");
        let refname = format!("refs/heads/{}", config_json.branch);
        let mut reference = repo.find_reference(&refname).unwrap();
        reference
            .set_target(fetch_commit.id(), "Fast-Forward")
            .unwrap();
        repo.set_head(&refname).unwrap();
        return repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()));
    }

    error!("Fast-forward only!");
    Err(Error::from_str("Fast-forward only!"))
}
