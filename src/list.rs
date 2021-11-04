use std::{io, fs};
// use std::time::SystemTime;
use crate::config::Config;
use humantime;

pub fn list_objects(_extra: Vec<String>, config: &Config) -> std::io::Result<()> {
    let files = fs::read_dir(&config.dirs.trash_dir)?
        .collect::<Result<Vec<_>, io::Error>>()?;
  
    println!("{0: <10}\t{1: <10}\t\t{2: <10}",
             "Name", "Date Modified", "Directory");


    for file in files {
        let metadata: Result<fs::Metadata, std::io::Error> = 
            fs::metadata(file.path());
        let is_dir: &str = match metadata {
            Ok(ref m) => match m.is_dir() {
                true  => "Yes",
                false => "No"
            },  _     => "NaN",
        };
        let mod_date: String = match metadata {
            Ok(m) => match m.modified() {
                Ok(t) => {
                    let mut s = humantime::format_rfc3339_seconds(t).to_string()
                                .replace("T", " ");
                    s.pop();
                    s
                },
                _ => String::from("NaN"),
            },  _ => String::from("NaN"),
        };
        
        println!("{0: <10}\t{1: <10}\t{2: <10}", 
            file.file_name().to_str().unwrap(),
            mod_date,
            // match humantime::format_rfc3339_seconds(&mod_date) {
            //     Ok(t) => t.to_string(),
            //     Err(_e) => String::from("NaN"),
            // },
            is_dir
        );
    }

    Ok(())
}
