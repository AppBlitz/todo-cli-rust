use std::{env::args, fmt::format, fs::File, io::Write, time, vec};
// struct to create structe of CLI
struct Todo {
    id: usize,
    description: String,
    status: String,
    created_at: String,
    update_at: String,
}
fn main() {
    let commands_principal = vec![
        "add",
        "delete",
        "update",
        "list",
        "mark-in-progress",
        "mark-in-done",
    ];

    #[warn(unused_variables)]
    let sub_commands = vec!["in-progres", "todo", "done"];
    let principal_argument = args().nth(1).expect("Command principal can not empty");
    let sub_command = args().nth(2).expect("sub command can not empty");
    let mut file_created = create_file();
    if verification_command_principal(commands_principal, &principal_argument) {
        if verification_command_secundary(sub_commands, &sub_command) {
        } else {
            if principal_argument == "add" {
                file_created.write_all(sub_command.as_bytes()).unwrap();
            }
        }
    } else {
        println!("[INFO] the command not found")
    }
    convert_vector_in_json();
}

fn verification_command_secundary(vector: Vec<&str>, command_secundary: &String) -> bool {
    let mut auxiliary: bool = false;
    for value_vector in vector {
        if value_vector == command_secundary {
            auxiliary = true;
        }
    }
    auxiliary
}

fn verification_command_principal(vector: Vec<&str>, command_initial: &String) -> bool {
    let mut auxiliary: bool = false;
    for i in vector {
        if i == command_initial {
            auxiliary = true;
        }
    }
    auxiliary
}

fn create_file() -> File {
    File::create("todo.json").unwrap()
}

fn convert_vector_in_json() {
    let format = format!(
        "[
id:{id}
]",
        id = 10
    );
    println!("{:?}", format.to_string())
}
