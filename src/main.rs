use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    env::args,
    fs::{self, File},
    io::Write,
    path::{self, Path},
    vec::Vec,
};

#[derive(Serialize, Deserialize)]
struct Todo {
    id: u64,
    description: String,
    status: String,
    create_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
}

fn main() {
    let commands_principal = vec![
        "add",
        "update",
        "delete",
        "list",
        "mark-in-progress",
        "mark-done",
    ];

    let sub_commands = vec!["done", "todo", "in-progres"];
    let commnand_principal = args().nth(1).expect("Arguement not found");
    let sub_command = args().nth(2).expect("Sub argument not found");
    let mut file_created: File = create_file();
    if verification_command_principal(commands_principal, commnand_principal) {
        if verification_command_principal(sub_commands, sub_command) {
        } else {
            let tasks = create_struct_task("description tasks 2".to_string());
            let json_save = serde_json::to_string(&tasks).unwrap();
            file_created.write_all(json_save.as_bytes()).unwrap();
            let path_file = path::absolute(Path::new("todo.json")).unwrap();
            // let vector_tasks: Vec<Todo> = Vec::new();
            let file_read = fs::read_to_string(path_file).unwrap();
            let solution: Vec<Todo> = serde_json::from_str(&file_read).unwrap();
            println!("the value json is:{:?}", solution[0].id)
        }
    }
}

fn create_file() -> File {
    File::create_new("todo.json").unwrap()
}

fn verification_command_principal(
    commands_principal: Vec<&str>,
    commnad_principal: String,
) -> bool {
    let mut auxiliary = false;
    for data_commands in &commands_principal {
        if data_commands == &commnad_principal {
            auxiliary = true
        }
    }
    auxiliary
}

fn create_struct_task(description_todo: String) -> Vec<Todo> {
    let utc: DateTime<Utc> = Utc::now();
    let mut vector_tasks: Vec<Todo> = Vec::new();
    let todo_one: Todo = Todo {
        id: 1,
        description: description_todo,
        status: "created".to_string(),
        create_at: utc,
        update_at: utc,
    };
    vector_tasks.push(todo_one);
    vector_tasks
}
