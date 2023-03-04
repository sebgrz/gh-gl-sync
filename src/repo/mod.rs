pub mod commit;
pub mod comparer;
pub mod error;

use self::{commit::Commit, error::ToRepositoryError};
use git2::{
    build::RepoBuilder, Cred, CredentialType, FetchOptions, Oid, PushOptions, RemoteCallbacks,
    Repository as Repository_git2, Sort,
};
use std::{collections::HashSet, path::Path};
use uuid::Uuid;

pub struct Repository {
    pub url: String,
    pub hash_url: String,
    repo: Option<Repository_git2>,
    ssh_private_key_path: String,
}

impl Repository {
    pub fn new(url: &str, ssh_private_key_path: &str) -> Repository {
        let uuid = Uuid::new_v4();
        let hash_url = sha256::digest(format!("{}{}", url, uuid));

        Repository {
            url: url.to_string(),
            hash_url,
            repo: None,
            ssh_private_key_path: ssh_private_key_path.into(),
        }
    }

    pub async fn clone_to_dir(&mut self) -> Result<(), error::RepositoryError> {
        let callbacks = self.auth_callbacks();

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let repo = RepoBuilder::new()
            .fetch_options(fo)
            .clone(
                self.url.as_str(),
                Path::new(format!("repo_{}", self.hash_url).as_str()),
            )
            .map_err(|e| e.to_repository_error())?;
        self.repo = Some(repo);
        Ok(())
    }

    pub fn clone_mirror_to_dir(&mut self) -> Result<(), error::RepositoryError> {
        let callbacks = self.auth_callbacks();
        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        let repo = RepoBuilder::new()
            .bare(true)
            .remote_create(|repo, name, url| {
                // [remote rejected] errors after mirroring a git repository - https://stackoverflow.com/a/34266401
                let remote = repo.remote_with_fetch(name, url, "+refs/heads/*:refs/heads/*")?;
                repo.remote_add_fetch(name, "+refs/tags/*:refs/tags/*")?;
                repo.remote_add_fetch(name, "+refs/change/*:refs/change/*")?;

                Ok(remote)
            })
            .fetch_options(fo)
            .clone(
                &self.url,
                Path::new(&format!("repo_{}_mirror", self.hash_url)),
            )
            .map_err(|e| e.to_repository_error())?;

        self.repo = Some(repo);

        Ok(())
    }

    pub fn push_mirror_to_repo(&mut self, url: &str) -> Result<(), error::RepositoryError> {
        let repo = self.repo.as_ref().unwrap();

        let mut remote = repo
            .remote("mirror", url)
            .map_err(|e| e.to_repository_error())?;
        let mut config = repo.config().map_err(|e| e.to_repository_error())?;
        config
            .set_bool("remote.mirror.mirror", true)
            .map_err(|e| e.to_repository_error())?;

        let refs: Vec<String> = repo
            .references()
            .map_err(|e| e.to_repository_error())?
            .map(|m| m.unwrap().name().unwrap().to_string())
            .collect();
        println!("{:?}", refs);

        let mut callbacks = self.auth_callbacks();
        callbacks.push_update_reference(|ref_name, status| {
            println!("{ref_name} - {status:?}");
            Ok(())
        });

        let mut po = PushOptions::new();
        po.remote_callbacks(callbacks);

        remote
            .push(&refs, Some(&mut po))
            .map_err(|e| e.to_repository_error())?;

        Ok(())
    }

    fn auth_callbacks<'a>(&'a self) -> RemoteCallbacks<'a> {
        let cred_callback = |_url: &str, _username: Option<&str>, cred_type: CredentialType| {
            if cred_type.is_ssh_key() {
                Cred::ssh_key("", None, Path::new(&self.ssh_private_key_path), None)
            } else {
                Err(git2::Error::from_str(&format!(
                    "only ssh private key authorization method is support but was {cred_type:?}"
                )))
            }
        };
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(cred_callback);

        callbacks
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
