use std::{env::args, path, vec};
// struct to create structe of CLI
struct CommandCli {
    command_principal: String,
    argument_twos: String,
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
    let args_command = CommandCli {
        command_principal: principal_argument,
        argument_twos: sub_command,
    };
    if verification_command_principal(commands_principal, &args_command.command_principal) {
        if verification_command_secundary(sub_commands, &args_command.argument_twos) {
        } else {
            if args_command.command_principal == "add" {}
        }
    }

    println!("absolute path is:{:?}", get_path_absolute())
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

fn create_file_extension_json() -> std::path::PathBuf {
    let path_file: path::PathBuf = get_path_absolute().into();
    let extension = path_file.with_extension("json");
    dbg!(extension)
}

fn get_path_absolute() -> path::PathBuf {
    let relative_path = path::Path::new("main.rs");
    path::absolute(relative_path).unwrap()
}
