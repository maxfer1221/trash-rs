use crate::config::Config;
use crate::trash::TrashFile;
use std::{fs, path::PathBuf};

pub fn perm_delete_files(files: Vec<String>, config: &Config) -> std::io::Result<()> {
    for file in files.iter().skip(2) {
        println!("{:?}", file);
        erase_metadata(&config.dirs.trash_info, &file)?;
    }
    Ok(())
}

fn erase_metadata(i: &PathBuf) -> std::io::Result<()> {
    let mut entries = fs::read_dir(i)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;
    
    println!("{:?}", entries);
    Ok(())
}
