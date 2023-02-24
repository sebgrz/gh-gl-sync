use std::env::args;

use repo::{comparer::compare_commits, error::RepositoryError, Repository};
use tokio::time::Instant;

use crate::repo::comparer::CommitDiff;

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

    let (commits_a_res, commits_b_res) =
        tokio::join!(repo_a.get_all_commits(), repo_b.get_all_commits());
    let commits_a = &mut commits_a_res?.into_iter().collect();
    let commits_b = &mut commits_b_res?.into_iter().collect();
    let diff = compare_commits(commits_a, commits_b);
    let duration = time.elapsed();

    // take not same
    let mut left_oids: Vec<String> = vec!();
    let mut right_oids: Vec<String> = vec!();
    diff
        .iter()
        .filter(|commit_diff| !matches!(**commit_diff, CommitDiff::SAME(_)))
        .map(|commit_diff| commit_diff.clone())
        .for_each(|commit_diff| { 
            match commit_diff {
            CommitDiff::LEFT(id) => left_oids.push(id),
            CommitDiff::RIGHT(id) =>  right_oids.push(id),
            _ => unreachable!("other than LEFT or RIGHT should not exists"),
        }});

    let left_commits = repo_a.get_commits_details(&left_oids);
    let right_commits = repo_b.get_commits_details(&right_oids);

    println!("left commits");
    left_commits.into_iter().for_each(|c| {
        println!("{:?}", c);
    });
    println!("right commits");
    right_commits.into_iter().for_each(|c| {
        println!("{:?}", c);
    });

    println!("Duration: {}", duration.as_millis());

    Ok(())
}
