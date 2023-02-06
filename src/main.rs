use std::env::args;

use repo::{error::RepositoryError, Repository, comparer::compare_commits};

pub mod repo;

fn main() -> Result<(), RepositoryError> {
    let repo_url_a = args()
        .nth(1)
        .expect("first git repository url is not provided");
    let repo_url_b = args()
        .nth(2)
        .expect("second git repository url is not provided");

    let mut repo_a = Repository::new(repo_url_a.as_str());
    repo_a.clone_to_dir()?;
    let mut repo_b = Repository::new(repo_url_b.as_str());
    repo_b.clone_to_dir()?;

    let commits_a = &mut repo_a.get_all_commits()?.into_iter().collect();
    let commits_b =  &mut repo_b.get_all_commits()?.into_iter().collect();
    let diff = compare_commits(commits_a, commits_b);

    diff.into_iter().for_each(|c| {
        println!("{:?}", c);
    });

    Ok(())
}
