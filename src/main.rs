use difference::{Difference, Changeset};
use std::fs;
use shellexpand::tilde;
use std::path::Path;

fn main() {
    let cow_thing = shellexpand::tilde("~/.config/nvim/init.vim");
    let cow_path = Path::new(cow_thing.as_ref());
    let config_nvim = fs::read_to_string(cow_path);
    println!("{:?}", config_nvim);

    // let changeset = Changeset::new(
    //     "big buttholes are the worst, or the best?", 
    //     "big vagface are the best, and the best?", 
    //     " ");

    // assert_eq!(changeset.diffs, vec![
    //     Difference::Same("big".to_string()),
    //     Difference::Rem("buttholes".to_string()),
    //     Difference::Add("vagface".to_string()),
    //     Difference::Same("are the".to_string()),
    //     Difference::Rem("worst, or".to_string()),
    //     Difference::Add("best, and".to_string()),
    //     Difference::Same("the best?".to_string()),
    // ])
}
