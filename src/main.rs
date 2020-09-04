mod file_actions;
mod git_actions;

use file_actions::actions::FileStore;
use git_actions::actions::{
    find_last_commit, git_add, git_commit, git_push};


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
    let mut are_changes = false;

    // for file in DOTFILES.iter() {
    //     let fs = FileStore::new(
    //         file.1.to_string(),
    //         file.0.to_string()
    //     );

    //     if !fs.compare_files() {
    //         are_changes = true;
    //         println!("Updates, writing changes:");
    //         fs.print();
    //         fs.write_backup();
    //     } else {
    //         println!("Files are the same:");
    //         fs.print();
    //     }

    //     if are_changes {
    //         println!("Do git stuff")
    //     }
    // }
}
