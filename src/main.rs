use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    env::args,
    fs::{self, File, OpenOptions},
    io::Write,
    path::{self, Path},
    vec::Vec,
};

#[derive(Serialize, Deserialize, Debug)]
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
    let sub_command = args().nth(2).unwrap_or(String::from("list"));
    let mut file_created: File = create_file();
    if verification_command_principal(commands_principal, &commnand_principal) {
        if verification_command_principal(sub_commands, &sub_command) || &sub_command == "list" {
            println!("{:?}", show_list_tasks(&sub_command))
        } else if &commnand_principal == "add" {
            let task_created = create_struct_task(sub_command);
            save_tasks(task_created, &mut file_created);
        } else if &commnand_principal == "update" {
        } else if &commnand_principal == "mark-in-progress" {
        } else if &commnand_principal == "mark-done" {
            mark_done_tasks(from_str(&sub_command).unwrap(), &mut file_created);
        }
    }
}

fn mark_done_tasks(id_task: u64, file: &mut File) {
    let (mut vector_tasks, _) = read_file();
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.status = "mark-done".to_string();
            task.update_at = Utc::now()
        }
    }
    save_tasks(vector_tasks, file);
}

fn save_tasks(tasks: Vec<Todo>, file: &mut File) {
    let json_sase = serde_json::to_string(&tasks).unwrap();
    file.write_all(json_sase.as_bytes()).unwrap()
}

fn show_list_tasks(status_list: &String) {
    let lists_tasks: Vec<Todo> = list_tasks(status_list);
    for show_list in lists_tasks {
        println!("{:?}", show_list)
    }
}

fn list_tasks(type_list: &String) -> Vec<Todo> {
    let (tasks_lists, size_tasks) = read_file();
    let mut tasks_list: Vec<Todo> = Vec::new();
    if size_tasks == 0 {
        tasks_lists
    } else if type_list != "list" {
        for lists in tasks_lists {
            if lists.status == *type_list {
                tasks_list.push(lists);
            }
        }
        tasks_list
    } else {
        tasks_lists
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
    let (mut vector_tasks, size_vector) = read_file();
    let todo_one: Todo = Todo {
        id: create_id_task(&vector_tasks, size_vector),
        description: description_todo,
        status: "todo".to_string(),
        create_at: utc,
        update_at: utc,
    };
    vector_tasks.push(todo_one);
    vector_tasks
}

fn read_file() -> (Vec<Todo>, u64) {
    let mut vector_tasks: Vec<Todo> = Vec::new();
    let path_absolut = path::absolute(Path::new("todo.json")).unwrap();
    let metadata = fs::metadata(&path_absolut).unwrap();
    if metadata.len() == 0 {
        (vector_tasks, 0)
    } else {
        let read_files = fs::read_to_string(&path_absolut).unwrap();
        vector_tasks = serde_json::from_str(&read_files).unwrap();
        (vector_tasks, metadata.len())
    }
}

fn create_id_task(tasks_vector: &Vec<Todo>, auxiliary_vector_tasks: u64) -> u64 {
    let mut auxiliary: u64 = 0;
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
