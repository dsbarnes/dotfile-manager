pub mod dot_manager{

    use difference::Changeset;
    use std::path::Path;
    use std::fs;

    pub struct DotManager {
        pub base_path: String,
        pub backup_path: String,
    }


    impl DotManager {
        pub fn new(base_path: String, backup_path: String) -> Self {
            DotManager{
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
                println!("{:?}", change.diffs);
                return false
            }
            true
        }

        pub fn write_backup(&self){
            // Move copy the base file,
            // write it to the backup file
        }
    }
}
