use std::{fs::rename, ffi::OsStr, path::PathBuf, env, time::SystemTime};
use crate::config::Config;
use crate::trash::trash_contains;
use humantime;

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

        if ext.eq("") {
            return false
        }
        
        println!("{} {}", config.copy_ext, &ext);
        if &ext[..(config.copy_ext.len())] == config.copy_ext {
            return true
        } false
    }
}

pub fn delete_files(files: Vec<String>, config: &Config) -> std::io::Result<()> {
    for file in files.iter().skip(2) {
        match delete_file(file, &config) {
            Err(e) => {
                println!("Error moving file: {:?}", e);
            },
            Ok(o, f) => write_metadata(o, f),
        }
    }
    Ok(())
}

fn delete_file(file: &String, config: &Config) -> std::io::Result<(PathBuf, PathBuf)> { 
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

    match rename(oname, fname) {
        Ok(_) => return Ok((oname, fname)),
        Err(e) => Err(e),
    }
}

fn write_metadata(o: PathBuf, f: PathBuf) -> std::io::Result<> {
    let t: SystemTime = std::time::now();
    let mut s: String = humantime::format_rfc3339_seconds(t).to_string()
                            .replace("T", " ");
    s.pop();

    let tf: TrashFile = trash::TrashFile { path: o, date: s };
    let toml_as_str: String = toml::to_string(tf);
    printf!("{}", toml_as_str);
}
