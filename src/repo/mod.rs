pub mod commit;
pub mod comparer;
pub mod error;

use self::{commit::Commit, error::ToRepositoryError};
use git2::{Oid, Repository as Repository_git2, Sort};
use std::collections::HashSet;
use uuid::Uuid;

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

    pub async fn clone_to_dir(&mut self) -> Result<(), error::RepositoryError> {
        let repo = Repository_git2::clone(self.url.as_str(), format!("repo_{}", self.hash_url))
            .map_err(|e| e.to_repository_error())?;
        self.repo = Some(repo);
        Ok(())
    }

    pub async fn get_all_commits(&self) -> Result<HashSet<String>, error::RepositoryError> {
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

    pub fn get_commits_details(&self, commits: &Vec<String>) -> Vec<Commit> {
        commits
            .iter()
            .map(|commit_id| {
                let commit_data = &self
                    .repo
                    .as_ref()
                    .unwrap()
                    .find_commit(Oid::from_str(&commit_id).unwrap())
                    .unwrap();
                Commit {
                    id: commit_id.into(),
                    time: commit_data.time().seconds(),
                }
            })
            .collect()
    }
}
