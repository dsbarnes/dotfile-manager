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
mod dot_manage;
mod git_actions;
// Get better at naming things
use dot_manage::dot_manager::DotManager;

// Take in arguments,
// add them to the array like this
// ideally, look through whole folders

// const DOTFILES: [(&str, &str); 3] = [
//     ("~/Documents/dotfiles/.config/nvim/init.vim",
//      "~/.config/nvim/init.vim"),
// 
//     ("~/Documents/dotfiles/.bash-git-prompt/theme/DSBarnes.bgptheme",
//      "/usr/local/opt/bash-git-prompt/share/themes/DSBarnes.bgptheme"),
// 
//     ("~/Documents/dotfiles/.bashrc",
//      "~/.bashrc"),
// ];

fn main() {
    let same = DotManager::new(
        DOTFILES[1].0.to_string(),
        DOTFILES[1].1.to_string()
    );

    let different = DotManager::new(
        DOTFILES[2].0.to_string(),
        DOTFILES[2].1.to_string()
    );

    same.print();
    different.print();
    // use shellexpand::tilde;
    // use shellexpand::tilde;

    let a = same.compare_files();
    println!("{}", a);

    let b = different.compare_files();
    println!("{}", b);
}
