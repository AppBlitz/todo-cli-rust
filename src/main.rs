use chrono::{DateTime, Utc};
use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{self, Path},
    vec::Vec,
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
enum StatusTaks {
    Todo,
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: usize,
    description: String,
    status: StatusTaks,
    create_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
}

fn main() {
    let result = Command::new("task-cli")
        .subcommand(
            Command::new("add")
                .alias("a")
                .arg(Arg::new("task").help("Description of task").required(false))
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("list")
                .alias("l")
                .arg(
                    Arg::new("status")
                        .help("Status of task")
                        .required(false)
                        .value_parser(["todo", "done", "InProgress"]),
                )
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("update")
                .arg(Arg::new("id_task"))
                .arg(Arg::new("description_tasks"))
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("delete")
                .alias("d")
                .arg(
                    Arg::new("task_id_delete")
                        .help("Id of taks for deleting")
                        .required(false),
                )
                .version("1.0.1")
                .about("delete taks for id "),
        )
        .subcommand(
            Command::new("mark-done")
                .arg(Arg::new("id").required(false).help("mark-done <id tasks>"))
                .about("Mark one task in done")
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("mark-in-progress")
                .arg(
                    Arg::new("id")
                        .help("mark-in-progress <id tasks>")
                        .required(false),
                )
                .about("Mark one task in progress")
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("mark-todo")
                .about("Mark one taks in todo")
                .arg(Arg::new("id").help("mark-todo <id tasks>").required(false))
                .version("1.0.1"),
        )
        .get_matches();

    let mut file_created = create_file();
    match result.subcommand() {
        Some(("add", sub_m)) => {
            match sub_m.get_one::<String>("task") {
                None => println!("Description task not found"),
                Some(description_tasks) => {
                    let task_created = create_struct_task(description_tasks);
                    save_tasks(task_created, &mut file_created);
                }
            };
        }
        Some(("list", sub_m)) => match sub_m.get_one::<String>("status") {
            None => {
                let (vector_tasks, _) = read_file();
                println!("{:?}", vector_tasks)
            }
            Some(status_tasks) => match status_tasks.as_str() {
                "todo" => {
                    search_status_tasks(StatusTaks::Todo);
                }
                "done" => {
                    search_status_tasks(StatusTaks::Done);
                }
                "InProgress" => {
                    search_status_tasks(StatusTaks::InProgress);
                }
                _ => {
                    println!("{:?}", "Value not allowed")
                }
            },
        },
        Some(("delete", sub_m)) => match sub_m.get_one::<String>("task_id_delete") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => delete_task_for_id(value, &mut create_file()),
                    Err(err) => eprintln!("{:?}, {:?}", "Value not allowed", err),
                };
            }
        },
        Some(("mark-done", sub_m)) => match sub_m.get_one::<String>("id") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => mark_done_tasks(value, &mut create_file()),
                    Err(err) => eprintln!("{:?}, {:?}", "Value not allowed", err),
                };
            }
        },
        Some(("mark-todo", sub_m)) => match sub_m.get_one::<String>("id") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => mark_todo_tasks(value, &mut create_file()),
                    Err(err) => eprintln!("{:?}, {:?}", "Value not allowed", err),
                };
            }
        },
        Some(("mark-in-progress", sub_m)) => match sub_m.get_one::<String>("id") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => mark_in_progress_tasks(value, &mut create_file()),
                    Err(err) => eprintln!("{:?}, {:?}", "Value not allowed", err),
                };
            }
        },
        Some(("update", sub_m)) => match sub_m.get_one::<String>("id_task") {
            None => {
                eprint!("Is necessary id of tasks")
            }
            Some(id_tasks) => match sub_m.get_one::<String>("description_tasks") {
                None => {
                    eprint!("Is necessary description of tasks")
                }
                Some(description) => match id_tasks.parse() {
                    Ok(task_id) => {
                        modification_description(task_id, description, &mut create_file());
                    }
                    Err(_) => {
                        eprintln!("Thi value of id not valid")
                    }
                },
            },
        },
        _ => {
            println!("commnad not found")
        }
    }
}

fn search_status_tasks(status: StatusTaks) {
    let (vector_tasks, _) = read_file();
    let mut vector_aux: Vec<Todo> = Vec::new();
    for task in vector_tasks {
        if task.status == status {
            vector_aux.push(task);
        }
    }
    println!("{:?}", vector_aux)
}

fn modification_description(id_task: usize, description_task: &str, file: &mut File) {
    let (mut vector_tasks, _) = read_file();
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.description = description_task.to_string();
            task.update_at = Utc::now();
        }
    }
    truncate_file(String::from("todo.json"));
    save_tasks(vector_tasks, file);
}

fn delete_task_for_id(id_task: usize, file: &mut File) {
    let (vector_tasks, _) = read_file();
    let mut tasks: Vec<Todo> = Vec::new();
    for task in vector_tasks {
        if task.id != id_task {
            tasks.push(task);
        }
    }
    truncate_file(String::from("todo.json"));
    save_tasks(tasks, file);
}
fn truncate_file(path: String) {
    std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
}

fn mark_done_tasks(id_task: usize, file: &mut File) {
    let (mut vector_tasks, _) = read_file();
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.status = StatusTaks::Done;
            task.update_at = Utc::now();
        }
    }
    truncate_file(String::from("todo.json"));
    save_tasks(vector_tasks, file);
}

fn mark_in_progress_tasks(id_task: usize, file: &mut File) {
    let (mut vector_tasks, _) = read_file();
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.status = StatusTaks::InProgress;
            task.update_at = Utc::now();
        }
    }
    truncate_file(String::from("todo.json"));
    save_tasks(vector_tasks, file);
}

fn mark_todo_tasks(id_task: usize, file: &mut File) {
    let (mut vector_tasks, _) = read_file();
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.status = StatusTaks::Todo;
            task.update_at = Utc::now();
        }
    }
    truncate_file(String::from("todo.json"));
    save_tasks(vector_tasks, file);
}

fn save_tasks(tasks: Vec<Todo>, file: &mut File) {
    let json_sase = serde_json::to_string(&tasks).unwrap();
    file.write_all(json_sase.as_bytes()).unwrap()
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

fn create_struct_task(description_todo: &String) -> Vec<Todo> {
    let utc: DateTime<Utc> = Utc::now();
    let (mut vector_tasks, size_vector) = read_file();
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

fn create_id_task(tasks_vector: &Vec<Todo>, auxiliary_vector_tasks: u64) -> usize {
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
