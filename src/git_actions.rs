pub mod actions{
    use std::path::Path;
    use git2::{
        Oid, Signature, Commit,
        ObjectType, Repository, Direction, Branch,
    };

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

    fn create_branch<'a>(
        repo: &'a Repository,
        branch_name: &str,
        target: &Commit,
        force: bool
    ) -> Result<Branch<'a>, git2::Error> {

        repo.branch(&branch_name, &target, force)
    }

    pub fn git_add(repo: &Repository, path: &Path
    ) -> Result<Oid, git2::Error>{

        let mut index = repo.index()?;
        index.add_path(path);
        index.write_tree()
    }
    

    pub fn git_commit(repo: &Repository, oid: Oid, message: &str){
        let signature = Signature::now(
            "Derek Barnes", "derekb0147@gmail.com")
            .unwrap();

        let parent_commit = find_last_commit(&repo)
            .unwrap();

        let tree = repo.find_tree(oid)
            .unwrap();

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit]
        );
    }


    pub fn git_push(repo: &Repository, url: &str, branch_name: &str
    ) -> Result<(), git2::Error> {

        let mut remote = match repo.find_remote("origin") {
            Ok(r) => r,
            Err(_) => repo.remote("origin", url)?,
        };
        remote.connect(Direction::Push)?;
        let refspec = format!("refs/heads/{}:refs/heads/{}",
                               branch_name, branch_name);

        remote.push(&[refspec], None)
    }
}
