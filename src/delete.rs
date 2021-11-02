use std::fs::rename;
use std::path::PathBuf;
use std::env;
use crate::config::Config;

pub fn delete_files(files: Vec<String>, config: &Config) -> std::io::Result<()> {
    let mut oname: PathBuf; 
    let mut fname: PathBuf;
    let mut temp_buf: PathBuf;
    for file in files.iter().skip(2) {
        temp_buf = PathBuf::from(&file);
        
        fname = config.trash_dir.clone();
        fname.push(temp_buf.file_name().unwrap());
        oname = env::current_dir().unwrap();
        oname.push(file);
        println!("{:?}", &fname);
        println!("{:?}", &oname);
        rename(oname, fname)?;
        
    }
    Ok(())
}

// fn write_metadata() -> Result<()> {
//     let toml_as_str: String = toml::to_string()
// }
