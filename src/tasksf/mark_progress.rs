use chrono::Utc;

use crate::{
    services::{
        create_file::{search_path_absolute, truncate_file},
        taks::save_tasks,
        tools::read_file,
    },
    util::enum_task::StatusTaks,
};
use std::fs::File;

pub fn mark_in_progress_tasks(id_task: usize, file: &mut File, name_file: &String) {
    let (mut vector_tasks, _) = read_file(name_file);
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.status = StatusTaks::InProgress;
            task.update_at = Utc::now();
        }
    }
    {
        truncate_file(search_path_absolute(name_file));
    };
    save_tasks(vector_tasks, file);
}
