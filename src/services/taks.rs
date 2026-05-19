use chrono::{DateTime, Utc};

use crate::{
    models::model_task::Todo,
    services::tools::{create_id_task, read_file},
    util::enum_task::StatusTaks,
};
use std::{fs::File, io::Write};

pub fn save_tasks(tasks: Vec<Todo>, file: &mut File) {
    let json_sase = serde_json::to_string(&tasks).unwrap();
    file.write_all(json_sase.as_bytes()).unwrap()
}
pub fn create_struct_task(description_todo: &String, name_file: String) -> Vec<Todo> {
    let utc: DateTime<Utc> = Utc::now();
    let (mut vector_tasks, size_vector) = read_file(&name_file);
    let id_task: usize = create_id_task(&vector_tasks, size_vector);
    let todo_one: Todo = Todo {
        id: id_task,
        description: description_todo.to_string(),
        status: StatusTaks::Todo,
        create_at: utc,
        update_at: utc,
    };
    vector_tasks.push(todo_one);
    vector_tasks
}
