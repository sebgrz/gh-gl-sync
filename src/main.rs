use std::{env::args, rc::Rc, sync::Arc, time::Duration};

use gh_gl_sync::{
    config, db, handlers,
    providers::Providers,
    repo::{comparer::compare_commits, comparer::CommitDiff, error::RepositoryError, Repository},
};
use rouille::{router, Response};
use tokio::{runtime::Runtime, time::Instant};

#[tokio::main]
async fn main() {
    let config_file_path = args()
        .nth(1)
        .expect("path to the configuration is required");

    let config = config::load(&config_file_path).unwrap_or_else(|e| {
        panic!("{:?}", e);
    });

    db::migrate(&config.database).await;

    let providers = Arc::new(Providers::new(&config).await);
    let runtime = Runtime::new().unwrap();

    rouille::start_server("0.0.0.0:3000", move |req| {
        let resp = router!(req,
            (POST) (/add-project) => {
                runtime.block_on(handlers::add_project_to_sync(providers.clone(), req))
            },
            (GET) (/sync-projects) => {
                runtime.spawn(handlers::sync_providers_projects(providers.clone()));
                Response::text("ok")
            },
            (GET) (/test/gh/create-project) => {
                runtime.block_on(handlers::create_gh_repo(providers.clone()))
            },
            (GET) (/test/gl/create-project) => {
                runtime.block_on(handlers::create_gl_repo(providers.clone()))
            },
            _ => Response::empty_404()
        );

        resp
    });
}

#[warn(dead_code)]
async fn main_old() -> Result<(), RepositoryError> {
    let repo_url_a = args()
        .nth(1)
        .expect("first git repository url is not provided");
    let repo_url_b = args()
        .nth(2)
        .expect("second git repository url is not provided");
    let ssh_path = args()
        .nth(3)
        .expect("repos ssh private key path is required");

    let time = Instant::now();
    let mut repo_a = Repository::new(&repo_url_a, &ssh_path);
    let mut repo_b = Repository::new(&repo_url_b, &ssh_path);
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
    let mut left_oids: Vec<String> = vec![];
    let mut right_oids: Vec<String> = vec![];
    diff.iter()
        .filter(|commit_diff| !matches!(**commit_diff, CommitDiff::SAME(_)))
        .map(|commit_diff| commit_diff.clone())
        .for_each(|commit_diff| match commit_diff {
            CommitDiff::LEFT(id) => left_oids.push(id),
            CommitDiff::RIGHT(id) => right_oids.push(id),
            _ => unreachable!("other than LEFT or RIGHT should not exists"),
        });

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

    let mut repo_mirror = Repository::new(&repo_url_a, &ssh_path);
    repo_mirror.clone_mirror_to_dir()?;
    repo_mirror.push_mirror_to_repo(&repo_url_b)?;

    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(())
}
