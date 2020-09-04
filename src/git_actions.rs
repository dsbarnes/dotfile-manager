pub mod actions{
    use git2::{
        Oid, Signature, Commit,
        ObjectType, Repository, Direction, Branch,
    };
    use std::path::Path;


    // Open an already existing repo
    pub fn git_open(path: &str) -> Result<Repository, git2::Error> {
        git2::Repository::open(&path)
    }

    // Need the parent commit to make a new commit
    pub fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
        let obj = repo
            .head()?
            .resolve()?
            .peel(ObjectType::Commit)?;
        obj
            .into_commit()
            .map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }

    fn create_branch<'a> (
        repo: &'a Repository,
        branch_name: &str,
        target: &Commit,
        force: bool) ->
        Result<Branch, git2::Error> {

        repo.branch(branch_name, target, force)
    }

    pub fn git_add(repo: &Repository, path: &Path) ->
        Result<Oid, git2::Error>{

        let mut index = repo.index()?;
        index.add_path(path);
        index.write_tree()
    }
    

    pub fn git_commit(repo: &Repository){
        let signature = Signature::now(
            "Derek Barnes", "derekb0147@gmail.com")?;
        let parent_commit = find_last_commit(&repo);
        let tree = repo.find_tree(oid);

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parent_commit
            );
    }


    pub fn add_and_commit(repo: &Repository, path: &Path, message: &str) ->
        Result<Oid, git2::Error> {

        let mut index = repo.index()?;
        index.add_path(path)?;
        let oid = index.write_tree()?;
        let signature = Signature::now(
            "Zbigniew Siciarz", "zbigniew@siciarz.net")?;
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
    pub fn git_push(repo: &Repository, url: &str) -> Result<(), git2::Error> {
        let mut remote = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => repo.remote("origin", url)?,
        };
        remote.connect(Direction::Push)?;
        remote.push(&["refs/heads/master:refs/heads/master"], None)
    }
}
