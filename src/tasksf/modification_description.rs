use std::fs::File;

use chrono::Utc;

use crate::services::{
    create_file::{search_path_absolute, truncate_file},
    taks::save_tasks,
    tools::read_file,
};
pub fn modification_description(
    id_task: usize,
    description_task: &str,
    file: &mut File,
    name_file: &String,
) {
    let (mut vector_tasks, _) = read_file(name_file);
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.description = description_task.to_string();
            task.update_at = Utc::now();
        }
    }
    {
        truncate_file(search_path_absolute(name_file));
    };
    save_tasks(vector_tasks, file);
}
