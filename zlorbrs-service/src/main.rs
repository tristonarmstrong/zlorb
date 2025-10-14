use git2::{BranchType, FetchOptions, Oid, Repository};
use log::error;
use serde::{Deserialize, Serialize};
use std::fs;
use zlorbrs_lib::config::Config;

#[derive(Serialize, Deserialize)]
struct ServiceConfig {
    sleep_time: u64,
}

fn main() {
    env_logger::init();

    let path_to_config_file_for_service = format!(
        "{}/.config/zlorbrs/service-config.json",
        std::env::home_dir().unwrap().to_str().unwrap()
    );

    let config_file = match std::fs::read_to_string(path_to_config_file_for_service) {
        Ok(a) => a,
        Err(e) => {
            error!("{e}");
            panic!("{e}");
        }
    };

    let config_data = serde_json::from_str::<ServiceConfig>(&config_file)
        .expect("Failed to convert config file to json string");

    loop {
        // get configs directory
        let dir_path = format!(
            "{}/.config/zlorbrs/configs",
            std::env::home_dir().unwrap().to_str().unwrap()
        );
        let directories = std::fs::read_dir(dir_path);
        // iterate the items in the directory
        directories.unwrap().for_each(|item_wrap| {
            // read the configs of each item
            let item = item_wrap.unwrap();
            let file_contents =
                fs::read_to_string(format!("{}/config.json", item.path().to_str().unwrap()))
                    .unwrap();
            let config_json = serde_json::from_str::<Config>(&file_contents).unwrap();
            let repo = Repository::open(config_json.path).expect("Failed to open repo");

            let mut remote = repo.find_remote("origin").expect("Remote not found");
            let mut fetch_options = FetchOptions::new();

            match remote.fetch(&["master"], Some(&mut fetch_options), None) {
                Ok(x) => {
                    println!("Fetched remote: {:?}", x);
                }
                Err(e) => {
                    println!("failed to fetch remote: {}", e);
                }
            };

            // check if repo has updates
            let local_branch = repo
                .find_branch("main", BranchType::Local)
                .expect("Local branch not found");
            let local_iod: Oid = local_branch
                .get()
                .target()
                .expect("Local branch has no target");

            let remote_branch_name = "origin/main";
            let remote_ref = repo
                .resolve_reference_from_short_name(remote_branch_name)
                .expect("Remote ref not found");
            let remote_iod: Oid = remote_ref.target().expect("Remote ref has no target");

            println!("local: {} - remote: {}", local_iod, remote_iod);

            if local_iod != remote_iod {
                println!(
                    "Remote has changes! Remote HEAD: {}, Local HEAD: {}",
                    remote_iod, local_iod
                );
            } else {
                println!("Up to date. No changes to pull.")
            }
        });
        take_a_nap(config_data.sleep_time);
    }
}

fn take_a_nap(sleep_time: u64) {
    std::thread::sleep(std::time::Duration::from_secs(sleep_time));
}
