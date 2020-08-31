mod file_actions;
// mod git_actions;
use file_actions::actions::FileStore;

/*
 * I pretty well only care about the settings in DOTFILES
 * 
 * One:
 *      Compare the backup file, to the base file
 *          if they are the same continue,
 *          else cp the base file contents to the backup file
 *      Once all comparisons are made
 *          create a new git branch and push it
 *          make MR
 *
 */

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
    for file in DOTFILES.iter() {
        let fs = FileStore::new(
            file.0.to_string(),
            file.1.to_string()
        );
        if !fs.compare_files() {
            // Copy the base -> backup
            // pull master, new branch, commit, pr
            println!("Copy dat baayssssss");
        }
    }
    println!("Happy Hacking");

}
