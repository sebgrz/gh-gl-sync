use std::env::args;

use repo::{error::RepositoryError, Repository, comparer::compare_commits};
use tokio::time::Instant;

pub mod repo;



#[tokio::main]
async fn main() -> Result<(), RepositoryError> {
    let repo_url_a = args()
        .nth(1)
        .expect("first git repository url is not provided");
    let repo_url_b = args()
        .nth(2)
        .expect("second git repository url is not provided");

    let time = Instant::now();
    let mut repo_a = Repository::new(repo_url_a.as_str());
    let mut repo_b = Repository::new(repo_url_b.as_str());
    let (res_a, res_b) = tokio::join!(repo_a.clone_to_dir(), repo_b.clone_to_dir());
    res_a?;
    res_b?;

    let (commits_a_res, commits_b_res) = tokio::join!(repo_a.get_all_commits(), repo_b.get_all_commits());
    let commits_a = &mut commits_a_res?.into_iter().collect();
    let commits_b =  &mut commits_b_res?.into_iter().collect();
    let diff = compare_commits(commits_a, commits_b);
    let duration = time.elapsed();

    diff.into_iter().for_each(|c| {
        println!("{:?}", c);
    });
    println!("Duration: {}", duration.as_millis());

    Ok(())
}
