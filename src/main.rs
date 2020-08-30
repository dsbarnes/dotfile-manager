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

struct DotfileCompare {
    base_path: String,
    backup_path: String,
 }

impl DotfileCompare {
    pub fn new(p1: String, p2: String) -> Self {
        DotfileCompare{
            base_path: p1,
            backup_path: p2,
        }
    }

    pub fn print(&self){
        println!("Base: {}", self.base_path);
        println!("Backup: {}", self.backup_path);
    }


    pub fn config_string(&self, input: &String) -> String{
        let init = shellexpand::tilde(&input);
        let path = Path::new(init.as_ref());
        fs::read_to_string(path).unwrap()
    }


    pub fn compare_files(&self) -> bool {
        let base_config = self.config_string(&self.base_path);
        let backup_config = self.config_string(&self.backup_path);
        let change = Changeset::new(&base_config, &backup_config, "\n");

        if change.diffs.len() > 1 {
            println!("The files are not the same");
            return false
        }
        true
    }

}


fn main() {
    println!("Happy hacking!");
    let test = DotfileCompare::new(
        "~/Documents/dotfiles/.config/nvim/init.vim".to_string(),
        "~/.config/nvim/init.vim".to_string()
    );
    test.print();
}
