use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};
pub fn create_file() {}

pub fn truncate_file(path_file: PathBuf) {
    let result = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path_file);
    match result {
        Ok(_) => {}
        Err(err) => {
            eprint!("{:?}", err)
        }
    }
}

pub fn open_file(path_file: PathBuf) -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open(path_file)
        .unwrap()
}

pub fn search_file() {}
