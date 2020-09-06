mod file_actions;
mod git_actions;

use file_actions::actions::FileStore;
use git_actions::actions::{
    git_open, find_last_commit, git_add, git_commit, git_push};

use std::path::Path;

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


fn main() {
    let repo_path = "/Users/dsbarnes/Documents/dotfiles";
    let repo = git_open(repo_path).unwrap();
    let mut were_changes = false;
    let mut oid;

    for file in DOTFILES.iter() {
        let fs = FileStore::new(
            file.1.to_string(), // Base_path
            file.0.to_string(), // Backup_path
        );

        if !fs.compare_files() {
            if !were_changes { were_changes = true };
            println!("Updates, writing changes and staging file:");
            fs.print();
            fs.write_backup();
            let path = Path::new(&fs.backup_path);
            oid = git_add(&repo, &path).unwrap();
        } else {
            println!("Files are the same:");
            fs.print();
        }

    }
    if were_changes {
        println!("Doing git stuff");
        // git_commit(&repo, oid, "Auto commit from dotfile manager");
        // git_push(&repo, "url", "branch");
    }
}
