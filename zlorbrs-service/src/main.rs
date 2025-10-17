use git2::{BranchType, Error, Oid, Remote, Repository};
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::{fs, io::Error as IoError, process::Stdio};
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
            error!("Failure reading config file to string: {e}");
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

        let dir_path = format!(
            "{}/.config/zlorbrs/configs",
            std::env::home_dir().unwrap().to_str().unwrap()
        );
        let directories = std::fs::read_dir(dir_path);
        if directories.is_err() {
            error!("There are no configuration files created yet");
            continue;
        }

        directories.unwrap().for_each(|item_wrap| {
            let item = item_wrap.unwrap();
            let file_contents =
                fs::read_to_string(format!("{}/config.json", item.path().to_str().unwrap()))
                    .unwrap();
            let config_json = serde_json::from_str::<Config>(&file_contents).unwrap();

            info!(" "); // this just makes logging easier to read
            info!("================ {} ===============", config_json.name);

            let repo = Repository::open(config_json.clone().path).expect("Failed to open repo");

            // ======= Fetching ==========
            // fast forward any changes if there is one
            let local_branch = repo
                .find_branch(&config_json.branch, BranchType::Local)
                .expect("Local branch not found");
            let local_iod: Oid = local_branch
                .get()
                .target()
                .expect("Local branch has no target");
            debug!("before iod: {local_iod}");

            let _ = fast_forward(&repo, &config_json);

            let remote_ref = repo
                .resolve_reference_from_short_name(&format!("origin/{}", config_json.branch))
                .expect("Remote ref not found");
            let remote_iod: Oid = remote_ref.target().expect("Remote ref has no target");
            debug!("remote iod: {remote_iod}");
            // ======= END ==========

            let dist_dir_exists = match std::fs::read_dir(format!("{}/dist", config_json.path)) {
                Ok(_) => true,
                Err(_) => false,
            };

            if !dist_dir_exists || local_iod != remote_iod {
                kick_off_build(&config_json);
            }
        });
    }
}

fn kick_off_build(config_json: &Config) {
    info!("Looks like we got some build pending, lets do that!");
    let path = format!("{}", config_json.path);
    debug!("Running build for: {}", config_json.path);

    let handle = std::thread::spawn(move || {
        let set_dir_res = std::env::set_current_dir(path.clone());
        if set_dir_res.is_err() {
            error!(
                "Failed to set the current directory: {}\nDir: {}",
                set_dir_res.err().unwrap(),
                path
            );
        }

        let bun_install_handle = std::process::Command::new("/root/.bun/bin/bun")
            .arg("i")
            .stdout(Stdio::piped())
            .output();
        if bun_install_handle.is_err() {
            error!(
                "Bun install handle failure: {}",
                bun_install_handle.err().unwrap()
            );
            return;
        }

        // TODO this needs to be populated via config
        let bun_handle = std::process::Command::new("/root/.bun/bin/bun")
            .arg("run")
            .arg("build")
            .stdout(Stdio::piped())
            .output();

        match bun_handle {
            Ok(h) => {
                debug!("got status: {:?}", h.status);
                match h.status.code() {
                    Some(0) => {
                        // create util split_to_debug_lines
                        let human_readable = String::from_utf8(h.stdout).unwrap();
                        let split_readable: Vec<&str> = human_readable.split("\n").collect();
                        for line in split_readable {
                            info!("build succeed: {:#?}", line);
                        }
                    }
                    Some(1) => {
                        error!("build error: {:?}", h.stderr);
                    }
                    _ => {}
                };
            }
            Err(e) => {
                error!("Total failure of bun_handle: {}", e);
            }
        };
    });

    handle.join().unwrap();
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
