use std::fs::rename;
use std::path::PathBuf;
use std::env;

pub fn delete_files(files: Vec<String>, trash_dir: &PathBuf) -> std::io::Result<()> {
    let mut fname: PathBuf;
    for file in files.iter().skip(2) {
        let mut oname: PathBuf = env::current_dir().unwrap();
        oname.push(file);
        fname = trash_dir.clone();
        fname.push(file);
        rename(oname, fname)?;
    }
    Ok(())
}

