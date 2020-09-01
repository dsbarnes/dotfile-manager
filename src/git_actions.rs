mod git_action{
    use std::env;
    use git2::{ Repository, Commit, ObjectType };

    struct GitAction {
        repo: String,
    }

    impl GitAction {
        pub fn new(){}
       
        pub fn add(){}

        pub fn commit(){}

        pub fn branch(){}
    }
}
