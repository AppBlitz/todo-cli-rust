use crate::{
    models::model_task::Todo,
    services::{
        create_file::{search_path_absolute, truncate_file},
        taks::save_tasks,
        tools::read_file,
    },
};
use std::fs::File;

pub fn delete_task_for_id(id_task: usize, file: &mut File, name_file: &String) {
    let (vector_tasks, _) = read_file(name_file);
    let mut tasks: Vec<Todo> = Vec::new();
    for task in vector_tasks {
        if task.id != id_task {
            tasks.push(task);
        }
    }
    {
        truncate_file(search_path_absolute(name_file));
    };
    save_tasks(tasks, file);
}
