pub mod actions{

    use difference::Changeset;
    use std::path::Path;
    use std::fs;

    pub struct FileStore {
        pub base_path: String,
        pub backup_path: String,
    }


    impl FileStore {
        pub fn new(base_path: String, backup_path: String) -> Self {
            FileStore {
                base_path,
                backup_path,
            }
        }

        pub fn print(&self){
            println!("Base: {}", self.base_path);
            println!("Backup: {}", self.backup_path);
        }


        pub fn file_to_string(&self, input: &String) -> String{
            let file = shellexpand::tilde(&input);
            let path = Path::new(file.as_ref());
            fs::read_to_string(path).unwrap()
        }


        pub fn compare_files(&self) -> bool {
            let base_config = self.file_to_string(&self.base_path);
            let backup_config = self.file_to_string(&self.backup_path);
            let change = Changeset::new(&base_config, &backup_config, "\n");

            if change.diffs.len() > 1 {
                println!("Copying {} -> {}", &self.base_path, &self.backup_path);
                return false
            }
            true
        }

        pub fn write_backup(&self){
            fs::copy(&self.base_path, &self.backup_path)
                .expect("failed to copy");
        }
    }
}
