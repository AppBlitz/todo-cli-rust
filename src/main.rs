mod commands;
mod models;
mod services;
mod util;
use crate::models::model_task::Todo;
use crate::services::{
    create_file::{open_file, truncate_file},
    error_formats::format_error_command,
    tools::create_id_task,
};
use crate::util::enum_task::StatusTaks;
use chrono::{DateTime, Utc};
use clap::{Arg, Command};
use std::{
    fs::{self, File},
    io::Write,
    path::{self, Path},
    vec::Vec,
};

fn main() {
    let result = Command::new("task-cli")
        .subcommand(
            Command::new("add")
                .alias("a")
                .arg(
                    Arg::new("task")
                        .help("Description")
                        .required(false)
                        .help(r#"Task description (e.g "buy milk")"#)
                        .value_name("description"),
                )
                .version("1.0.2"),
        )
        .subcommand(
            Command::new("list")
                .alias("l")
                .arg(
                    Arg::new("status")
                        .help("status of task ( e.g todo) ")
                        .required(false)
                        .value_parser(["todo", "done", "InProgress"])
                        .value_name("status"),
                )
                .version("1.0.2"),
        )
        .subcommand(
            Command::new("update")
                .arg(
                    Arg::new("id_task")
                        .value_name("id")
                        .help(" id of task ( e.g 0)"),
                )
                .arg(
                    Arg::new("description_tasks")
                        .value_name("description")
                        .help(r#" description task  ( e.g "buy list kitchend")"#),
                )
                .version("1.0.2"),
        )
        .subcommand(
            Command::new("delete")
                .alias("d")
                .arg(
                    Arg::new("task_id_delete")
                        .help("id of task delete ( e.g 0)")
                        .value_name("id")
                        .required(false),
                )
                .version("1.0.2")
                .about("delete taks for id "),
        )
        .subcommand(
            Command::new("mark-done")
                .arg(
                    Arg::new("id")
                        .required(false)
                        .help("id of task ( e.g 0)")
                        .value_name("id"),
                )
                .about("Mark one task in done")
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("mark-in-progress")
                .arg(
                    Arg::new("id")
                        .help(" id of task ( e.g 0)")
                        .required(false)
                        .value_name("id"),
                )
                .about("Mark one task in progress")
                .version("1.0.1"),
        )
        .subcommand(
            Command::new("mark-todo")
                .about("Mark one taks in todo")
                .arg(
                    Arg::new("id")
                        .help("id of task ( e.g 0) ")
                        .required(false)
                        .value_name("id"),
                )
                .version("1.0.1"),
        )
        .get_matches();

    let mut file_created = create_file();
    match result.subcommand() {
        Some(("add", sub_m)) => {
            match sub_m.get_one::<String>("task") {
                None => eprintln!(
                    "{}",
                    format_error_command(
                        "Invalid description",
                        "Tha value of description be string and not null ",
                        "use:./target/release/todo-cli-rusth --help <value>"
                    )
                ),
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
                    Err(_) => eprintln!(
                        "{}",
                        format_error_command(
                            "Invalid task ID",
                            "The provided value is not a valid number",
                            "use: ./target/release/todo-cli-rust --help <value>"
                        )
                    ),
                };
            }
        },
        Some(("mark-done", sub_m)) => match sub_m.get_one::<String>("id") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => mark_done_tasks(value, &mut create_file()),
                    Err(_) => eprintln!(
                        "{}",
                        format_error_command(
                            "The status not allowed",
                            "The status not be null o different",
                            "use:./target/release/todo-cli-rust --help <value>"
                        )
                    ),
                };
            }
        },
        Some(("mark-todo", sub_m)) => match sub_m.get_one::<String>("id") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => mark_todo_tasks(value, &mut create_file()),
                    Err(_) => eprintln!(
                        "{}",
                        format_error_command(
                            "Status not search",
                            "Status of task not allowed",
                            "use:./target/release/todo-cli-rust --help update"
                        )
                    ),
                };
            }
        },
        Some(("mark-in-progress", sub_m)) => match sub_m.get_one::<String>("id") {
            None => {}
            Some(id_task) => {
                match id_task.parse() {
                    Ok(value) => mark_in_progress_tasks(value, &mut create_file()),
                    Err(_) => eprintln!(
                        "{}",
                        format_error_command(
                            "Invalid status tasks",
                            "Status task not allowed",
                            "use: ./target/release/todo-cli-rust --help <value>"
                        )
                    ),
                };
            }
        },
        Some(("update", sub_m)) => match sub_m.get_one::<String>("id_task") {
            None => {
                eprintln!(
                    "{}",
                    format_error_command(
                        "Invalid search value",
                        "Value cannot be null",
                        "Use:./target/release/todo-cli-rust --help <VALUE>"
                    )
                )
            }
            Some(id_tasks) => match sub_m.get_one::<String>("description_tasks") {
                None => {
                    eprintln!(
                        "{}",
                        format_error_command(
                            "Description of task not found",
                            "Value not cannot be null",
                            "use:./target/release/todo-cli-rust --help <value>"
                        )
                    )
                }
                Some(description) => match id_tasks.parse() {
                    Ok(task_id) => {
                        modification_description(task_id, description, &mut create_file());
                    }
                    Err(_) => {
                        eprintln!(
                            "{}",
                            format_error_command(
                                "Invalid task ID",
                                "The provided value is not a valid number",
                                "The <id> must be a numeric value (e.g. 0,1, 2, 3)"
                            )
                        )
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
    {
        let path_file = String::from("todo.json");

        // truncate_file();
    };
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
    {
        let path_file = String::from("todo.json");
        // truncate_file(path_file);
    };
    save_tasks(tasks, file);
}

fn mark_done_tasks(id_task: usize, file: &mut File) {
    let (mut vector_tasks, _) = read_file();
    for task in &mut vector_tasks {
        if task.id == id_task {
            task.status = StatusTaks::Done;
            task.update_at = Utc::now();
        }
    }
    {
        let path_file = String::from("todo.json");
        // truncate_file(jk);
    };
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
    {
        let path_file = String::from("todo.json");
        // truncate_file(jk);
    };
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
    {
        let path_file = String::from("todo.json");
        // truncate_file(jk);
    };
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
        open_file(path_file)
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
