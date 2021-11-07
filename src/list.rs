use std::{io::{self, Error, ErrorKind, BufReader, Read}, fs};
use crate::config::Config;
use crate::trash::TrashFile;

pub fn list_objects(_extra: Vec<String>, config: &Config) -> Result<(), Error> {
    let files = fs::read_dir(&config.dirs.trash_info)?
        .collect::<Result<Vec<_>, io::Error>>()?;
  
    println!("{0: <10}\t\t{1: <10}\t\t{2: <10}",
             "Name", "Date Modified", "Original Location");


    for file in files { 
        let mut buf_reader = 
            BufReader::new(fs::File::open(file.path())?);
        let mut contents = String::new(); 
        buf_reader.read_to_string(&mut contents)?;
        
        let trash_file: TrashFile = toml::from_str(&contents)?;
        let mod_date: String = trash_file.date;
        
        let o_path: String = trash_file.path.into_os_string().into_string()
            .map_err(|_e| Error::new(ErrorKind::Other, "Could not convert OsString to string"))?;

        println!("{0: <10}\t\t{1: <10}\t{2: <10}",
            file.file_name().to_str().unwrap_or(""),
            mod_date,
            o_path
        ); 
    }

    Ok(())
}
