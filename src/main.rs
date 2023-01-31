use std::env::args;

use git2::{Repository, Sort, Error};

fn main() -> Result<(), Error> {
    let repo_url = args().nth(1).expect("git repository url is not provided");
    let repo = Repository::clone(repo_url.as_str(), "test")?;
    let mut revwalk = repo.revwalk()?;
      
    revwalk.set_sorting(Sort::NONE)?;
    revwalk.push_head()?;

    for oid in revwalk {
        let commit = repo.find_commit(oid?).unwrap();
        println!("{} - {}", commit.id().to_string(), commit.summary().unwrap_or_else(|| "")) 
    }

    Ok(())
}
