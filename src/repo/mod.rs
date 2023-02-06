pub mod comparer;
pub mod error;

use self::error::ToRepositoryError;
use git2::{Repository as Repository_git2, Sort};
use uuid::Uuid;
use std::collections::HashSet;

pub struct Repository {
    pub url: String,
    pub hash_url: String,
    repo: Option<Repository_git2>,
}

impl Repository {
    pub fn new(url: &str) -> Repository {
        let uuid = Uuid::new_v4();
        let hash_url = sha256::digest(format!("{}{}", url, uuid));

        Repository {
            url: url.to_string(),
            hash_url,
            repo: None,
        }
    }

    pub fn clone_to_dir(&mut self) -> Result<(), error::RepositoryError> {
        let repo = Repository_git2::clone(self.url.as_str(), format!("repo_{}", self.hash_url))
            .map_err(|e| e.to_repository_error())?;
        self.repo = Some(repo);
        Ok(())
    }

    pub fn get_all_commits(&self) -> Result<HashSet<String>, error::RepositoryError> {
        let mut commits: HashSet<String> = HashSet::new();
        let mut revwalk = self
            .repo
            .as_ref()
            .unwrap()
            .revwalk()
            .map_err(|e| e.to_repository_error())?;

        revwalk
            .set_sorting(Sort::TIME)
            .map_err(|e| e.to_repository_error())?;

        revwalk
            .push_glob("refs/*")
            .map_err(|e| e.to_repository_error())?;

        for oid in &mut revwalk {
            commits.insert(oid.unwrap().to_string());
        }

        Ok(commits)
    }
}
