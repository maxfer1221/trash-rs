use std::{fs::rename, ffi::OsStr, path::PathBuf, env};
use crate::config::Config;
use crate::trash::trash_contains;

trait Rename {
    fn is_copy_obj(&self, config: &Config) -> bool;
    fn rename_dup(self, copy_ext: &String, file_count: u64) -> PathBuf;
}

impl Rename for PathBuf {
    fn rename_dup(mut self, copy_ext: &String, file_count: u64) -> PathBuf {
        self.set_extension(
            format!("{}{}", copy_ext, file_count)
            );
        self
    }

    fn is_copy_obj(&self, config: &Config) -> bool {  
        let ext: String = match self.extension() {
            Some(s) => match s.to_os_string().into_string() {
                Ok(s) => s,
                _ => "".into(),
            },
            _ => "".into(),
        };

        if &ext[..(config.copy_ext.len())] == config.copy_ext {
            return true
        } false
    }
}

pub fn delete_files(files: Vec<String>, config: &Config) -> std::io::Result<()> {
    for file in files.iter().skip(2) {
        let result  = delete_file(file, &config);
        match result {
            Err(e) => {
                println!("Error moving file: {:?}", e);
            },
            _ => {},
        }
    }
    Ok(())
}

fn delete_file(file: &String, config: &Config) -> std::io::Result<()> { 
    let mut oname: PathBuf; 
    let mut fname: PathBuf;
    let temp_buf: PathBuf;
    
    temp_buf = PathBuf::from(&file);
    fname = config.dirs.trash_dir.clone();
    fname.push(temp_buf.file_name().unwrap());
    
    if fname.is_copy_obj(config) {
        fname = match fname.clone().file_name() {
            Some(n) => {
                fname.rename_dup(
                    &config.copy_ext, 
                    trash_contains(n, &config.dirs.master_dir).1)
            },
            None => PathBuf::new().rename_dup(
                &config.copy_ext, trash_contains(
                    OsStr::new(""), &config.dirs.trash_info).1),
        };
    } else {
        let (tc, c): (bool, u64) = trash_contains(
            match fname.file_name(){
                Some(n) => n,
                None => OsStr::new(""),
            }, &config.dirs.master_dir);

        fname = match tc {
            true => fname.rename_dup(&config.copy_ext, c),
            false => fname,
        };
    }

    oname = env::current_dir().unwrap();
    oname.push(file);

    rename(oname, fname) 
}

// fn write_metadata() -> Result<()> {
//     let toml_as_str: String = toml::to_string()
// }
