use git2::Repository;
use log::error;

pub fn fetch_update(repo_path: String) {
    let repo: Option<Repository> = match Repository::open(repo_path) {
        Ok(repo) => Some(repo),
        Err(e) => {
            error!("failed to init: {}", e);
            None
        }
    };
}
