use difference::{Difference, Changeset};
use std::fs;
use shellexpand::tilde;
use std::path::Path;

/*
 * Get the file from it's normal location
 * Compare that file to the one in the git repo
 * if different move the contents of the normal location into the git repo
 * else continue
 * for each of the files I want to check
 * 
 * git commit
 * push a git branch called dotfile-update-{timestamp}
 * 
 */
const DOTFILES: [(&str, &str); 3] = [
    ("~/Documents/dotfiles/.config/nvim/init.vim",
     "~/.config/nvim/init.vim"),

    ("~/Documents/dotfiles/.bash-git-prompt/theme/DSBarnes.bgptheme",
     "/usr/local/opt/bash-git-prompt/share/themes"),

    ("~/Documents/dotfiles/.bashrc",
     "~/.bashrc"),
];


fn read_path_to_string(input: String) -> String{
    let init = shellexpand::tilde(&input);
    let path = Path::new(init.as_ref());
    fs::read_to_string(path).unwrap()
}


fn create_changeset(f1: String, f2: String) -> Changeset {
    Changeset::new(
        &read_path_to_string(f1), 
        &read_path_to_string(f2), 
        "\n"
    )
}


fn main() {
    println!("Happy hacking!");
}
