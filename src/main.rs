use clap::{Arg, Command};
use std::fs::File;
use utils::services::taks::{create_struct_task, save_tasks};
use utils::services::tools::{read_file, search_status_tasks};
use utils::services::{create_file::create_file, error_formats::format_error_command};
use utils::tasksf::delete_task::delete_task_for_id;
use utils::tasksf::mark_done::mark_done_tasks;
use utils::tasksf::mark_progress::mark_in_progress_tasks;
use utils::tasksf::mark_todo::mark_todo_tasks;
use utils::tasksf::modification_description::modification_description;
use utils::util::enum_task::StatusTaks;

fn main() {
    let file_name: String = String::from("task.json");
    let mut file_created: File = create_file(&file_name);

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
                    let task_created = create_struct_task(description_tasks, file_name);
                    save_tasks(task_created, &mut file_created);
                }
            };
        }
        Some(("list", sub_m)) => match sub_m.get_one::<String>("status") {
            None => {
                let (vector_tasks, _) = read_file(&file_name);
                println!("{:?}", vector_tasks)
            }
            Some(status_tasks) => match status_tasks.as_str() {
                "todo" => {
                    search_status_tasks(StatusTaks::Todo, &file_name);
                }
                "done" => {
                    search_status_tasks(StatusTaks::Done, &file_name);
                }
                "InProgress" => {
                    search_status_tasks(StatusTaks::InProgress, &file_name);
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
                    Ok(value) => delete_task_for_id(value, &mut file_created, &file_name),
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
                    Ok(value) => mark_done_tasks(value, &mut file_created, &file_name),
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
                    Ok(value) => mark_todo_tasks(value, &mut file_created, &file_name),
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
                    Ok(value) => mark_in_progress_tasks(value, &mut file_created, &file_name),
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
                        modification_description(
                            task_id,
                            description,
                            &mut file_created,
                            &file_name,
                        );
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
