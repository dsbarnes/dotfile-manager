mod file_actions;
// mod git_actions;
use file_actions::actions::FileStore;
use std::env;
use std::path::Path;
use git2::{ Repository, Commit, ObjectType, Oid };
// use std::time;

const DOTFILES: [(&str, &str); 4] = [
    ("~/Documents/dotfiles/.config/nvim/init.vim",
     "~/.config/nvim/init.vim"),

    ("~/Documents/dotfiles/.bash-git-prompt/theme/DSBarnes.bgptheme",
     "/usr/local/opt/bash-git-prompt/share/themes/DSBarnes.bgptheme"),

    ("~/Documents/dotfiles/.bashrc",
     "~/.bashrc"),

    ("~/Documents/dotfiles/.bash_profile",
     "~/.bash_profile"),
];

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn display_commit(commit: &Commit) {
    println!("commit {}\nAuthor: {}\n\n    {}",
             commit.id(),
             commit.author(),
             commit.message().unwrap_or("no commit message"));
}

fn main() {
    // let repo = Repository::open(".").unwrap();
    // let lc = find_last_commit(&repo).unwrap();
    // display_commit(&lc);

    for file in DOTFILES.iter() {
        let fs = FileStore::new(
            file.1.to_string(),
            file.0.to_string()
        );

        fs.print();
        println!("");

        // if the files are not the same:
        if !fs.compare_files() {
            println!("writing {} to {}", file.1, file.0);
            fs.write_backup();
            
            // pull master, new branch, commit, pr
            println!("do git stuff now");
        } else {
            println!("The files are the same");
        }
    }
}
