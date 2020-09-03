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
                base_path: shellexpand::tilde(&base_path)
                    .as_ref()
                    .to_string(),
                backup_path: shellexpand::tilde(&backup_path)
                    .as_ref()
                    .to_string(),
            }
        }

        pub fn print(&self){
            println!("Base: {}", self.base_path);
            println!("Backup: {}", self.backup_path);
        }


        pub fn compare_files(&self) -> bool {
            let base_config = fs::read_to_string(&self.base_path)
                .expect("failed to read base config");
            let backup_config = fs::read_to_string(&self.backup_path)
                .expect("failed to read backup path");

            let change = Changeset::new(&base_config, &backup_config, "\n");
            if change.diffs.len() > 1 {
                return false
            }
            true
        }

        pub fn write_backup(&self){
            fs::copy(&self.base_path, &self.backup_path)
                .expect("WRITE_BACKUP: failed to copy");

            println!("Successfully copied {} to {}",
                &self.base_path, &self.backup_path);
        }
    }
}
