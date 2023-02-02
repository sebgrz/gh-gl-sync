use std::env::args;
use git2::{BranchType, Error, Repository, Sort};

fn main() -> Result<(), Error> {
    let repo_url = args().nth(1).expect("git repository url is not provided");

    let mut branch_refs: Vec<(String, String)> = Vec::new();
    let mut oids: Vec<String> = Vec::new();
    let repo = Repository::clone(repo_url.as_str(), "test")?;
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TIME)?;
    revwalk.push_glob("refs/*")?;
    for oid in &mut revwalk {
        oids.push(oid?.to_string());
    }

    let branches = repo.branches(Some(BranchType::Remote))?;
    for b in branches {
        let (branch, _) = b?;
        let refr = branch.get();
        let object = refr.peel(git2::ObjectType::Commit)?;
        let branch_commit = object.as_commit().unwrap();

        let ref_oid = branch_commit.id().to_string();
        let ref_name = refr.name().unwrap();

        (&mut branch_refs).push((ref_oid, ref_name.to_string()));
    }

    (&mut oids).into_iter().for_each(|id| {
        let ref_name = (&mut branch_refs)
            .into_iter()
            .filter(|r| r.0 == *id)
            .map(|m| m.1.to_string())
            .next()
            .unwrap_or("".to_string());
        println!("{} - {}", id, ref_name);
    });

    Ok(())
}
