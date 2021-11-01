use std::fs::rename;
use std::path::PathBuf;

pub fn delete_files(files: Vec<String>, trash_dir: &PathBuf) -> std::io::Result<()> {
    println!("{:?}", files);
    let mut fname: PathBuf;
    for file in files.iter().skip(2) {
        fname = trash_dir.clone();
        fname.push(file);
        rename(file, fname)?;
    }
    Ok(())
}

