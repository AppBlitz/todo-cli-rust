use std::{
    fs::{File, OpenOptions, exists},
    path::{PathBuf, absolute},
};
pub fn create_file(name_file: &String) -> File {
    let path_file: PathBuf = absolute(name_file).unwrap();
    if !exists(&path_file).unwrap() {
        File::create_new(&path_file).unwrap()
    } else {
        open_file(path_file)
    }
}

pub fn search_path_absolute(name_file: &String) -> PathBuf {
    if !exists(name_file).unwrap() {
        create_file(name_file);
    }
    absolute(name_file).unwrap()
}

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
