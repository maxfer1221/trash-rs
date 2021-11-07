use crate::config::Config;
// use std::fs;

pub trait PermDelete {
    fn perm_delete(&self, config: &Config) -> std::io::Result<()>;
}

impl PermDelete for String { 
    fn perm_delete(&self, config: &Config) -> std::io::Result<()> {
        // fix_master_metadata();
        Ok(())
    }
}

impl PermDelete for Vec<String> {
    fn perm_delete(&self, config: &Config) -> std::io::Result<()> {
        for file in self {
            String::perm_delete(file, config)?;
        }
        Ok(())
    }
}

pub fn perm_delete_files<T: PermDelete>(files: T, config: &Config) -> std::io::Result<()> {
    files.perm_delete(config)?;
    Ok(())
}
