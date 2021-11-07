use serde::{Serialize, Deserialize};
use std::path::PathBuf;
    
#[derive(Serialize, Deserialize, Debug)]
pub struct TrashFile {
    pub path: PathBuf,
    pub date: String,
}

impl TrashFile {
    pub fn new(p: &PathBuf, d: &String) -> TrashFile {
        TrashFile { path: p.clone(), date: d.clone() }
    }
}
