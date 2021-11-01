use std::fs::{File, rename};
use std::process::Command;
use 

mod delete {

    pub fn delete_files(files: &[str], args: &[str]) -> Result<String, Err>{
        for file in files {
            rename(file, TRASH_DIR);
        }
    }
}
