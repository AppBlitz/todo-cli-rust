use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    env::args,
    fs::{self, File, OpenOptions},
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
    if verification_command_principal(commands_principal, &commnand_principal) {
        if verification_command_principal(sub_commands, &sub_command) {
        } else {
            let json_sase =
                serde_json::to_string(&create_struct_task(sub_command.to_string())).unwrap();
            file_created.write_all(json_sase.as_bytes()).unwrap()
        }
    }
}

fn create_file() -> File {
    let name_file: String = String::from("todo.json");
    let path_file = path::absolute(Path::new(&name_file)).unwrap();
    if !fs::exists(&path_file).unwrap() {
        File::create_new(&path_file).unwrap()
    } else {
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(path_file)
            .unwrap()
    }
}

fn verification_command_principal(
    commands_principal: Vec<&str>,
    commnad_principal: &String,
) -> bool {
    let mut auxiliary = false;
    for data_commands in commands_principal {
        if data_commands == commnad_principal {
            auxiliary = true
        }
    }
    auxiliary
}

fn create_struct_task(description_todo: String) -> Vec<Todo> {
    let utc: DateTime<Utc> = Utc::now();
    let mut vector_tasks: Vec<Todo> = read_file();
    let todo_one: Todo = Todo {
        id: create_id_task(&vector_tasks),
        description: description_todo,
        status: "created".to_string(),
        create_at: utc,
        update_at: utc,
    };
    vector_tasks.push(todo_one);
    vector_tasks
}

fn read_file() -> Vec<Todo> {
    let path_absolut = path::absolute(Path::new("todo.json")).unwrap();
    let read_files = fs::read_to_string(path_absolut).unwrap();
    let vector_tasks: Vec<Todo> = serde_json::from_str(&read_files).unwrap();
    vector_tasks
}

fn create_id_task(tasks_vector: &Vec<Todo>) -> u64 {
    let mut auxiliary: u64 = 0;
    for taks in tasks_vector {
        if taks.id > auxiliary {
            auxiliary = taks.id
        }
    }
    auxiliary + 1
}
