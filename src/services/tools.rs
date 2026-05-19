use crate::services::create_file::search_path_absolute;
use crate::{models::model_task::Todo, util::enum_task::StatusTaks};
use std::fs::{metadata, read_to_string};

pub fn create_id_task(tasks_vector: &Vec<Todo>, auxiliary_vector_tasks: u64) -> usize {
    let mut auxiliary: usize = 0;
    if auxiliary_vector_tasks == 0 {
        auxiliary
    } else {
        for taks in tasks_vector {
            if taks.id > auxiliary {
                auxiliary = taks.id
            }
        }
        auxiliary + 1
    }
}
pub fn search_status_tasks(status: StatusTaks, name_file: &String) {
    let (vector_tasks, _) = read_file(name_file);
    let mut vector_aux: Vec<Todo> = Vec::new();
    for task in vector_tasks {
        if task.status == status {
            vector_aux.push(task);
        }
    }
    println!("{:?}", vector_aux)
}
pub fn read_file(name_file: &String) -> (Vec<Todo>, u64) {
    let mut vector_tasks: Vec<Todo> = Vec::new();
    let path_absolut = search_path_absolute(&name_file);
    let metadata = metadata(&path_absolut).unwrap();
    if metadata.len() == 0 {
        (vector_tasks, 0)
    } else {
        let read_files = read_to_string(&path_absolut).unwrap();
        vector_tasks = serde_json::from_str(&read_files).unwrap();
        (vector_tasks, metadata.len())
    }
}
