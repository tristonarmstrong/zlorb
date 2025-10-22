pub mod config;

use log::error;

pub fn get_home_dir() -> String {
    let home_dir = match std::env::home_dir() {
        Some(x) => String::from(x.to_str().unwrap()),
        None => {
            error!("Failed to get the home directory");
            panic!("Program exited due to previous error");
        }
    };
    home_dir
}
