use std::fs::rename;
use std::path::PathBuf;
use std::env;
use crate::config::Config;

pub fn delete_files(files: Vec<String>, config: &Config) -> std::io::Result<()> {
    let mut oname: PathBuf; 
    let mut fname: PathBuf;
    for file in files.iter().skip(2) {
        fname = config.trash_dir.clone();
        fname.push(file);
        oname = env::current_dir().unwrap();
        oname.push(file);
        // println!("{:?}", fname);
        // println!("{:?}", oname);
        // println!("{:?}", metadata(&oname).unwrap());
        rename(oname, fname)?;
    }
    Ok(())
}

