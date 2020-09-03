mod git_action{
    use git2::{Oid, Signature, Commit, ObjectType, Repository, Direction};
    use std::path::Path;


    // Need the parent commit to make a new commit
    fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
        let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }


    // Will need split into two function, one for add, one for commit
    // Fairly stright forward
    fn add_and_commit(repo: &Repository, path: &Path, message: &str) -> Result<Oid, git2::Error> {
        let mut index = repo.index()?;
        index.add_path(path)?;
        let oid = index.write_tree()?;
        let signature = Signature::now("Zbigniew Siciarz", "zbigniew@siciarz.net")?;
        let parent_commit = find_last_commit(&repo)?;
        let tree = repo.find_tree(oid)?;
        repo.commit(Some("HEAD"), //  point HEAD to our new commit
                    &signature, // author
                    &signature, // committer
                    message, // commit message
                    &tree, // tree
                    &[&parent_commit]) // parents
    }


    // Needs the branch name
    fn push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
        let mut remote = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => repo.remote("origin", url)?,
        };
        remote.connect(Direction::Push)?;
        remote.push(&["refs/heads/master:refs/heads/master"], None)
    }
}
