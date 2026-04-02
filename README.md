## Acknowledgements

This project is based on a challenge from roadmap.sh:
[ fileTask Tracker Challenge - roadmap.sh](https://roadmap.sh/projects/task-tracker)


# Installation

## Clone repository 
```bash
git clone git clone https://github.com/AppBlitz/todo-cli-rust.git 
cd todo-cli-rust
```


## Build the project using Cargo:
```bash
cargo build --release
```

## Run executable:

```bash
./target/release/todo-cli-rust 
```


# Usage


## create new tasks:
 for creating new task use command *add*:
This command receives the task description this is *<description tasks>*

```bash
./target/release/todo-cli-rust add <description tasks>
```


## list all tasks:
for list all tasks use *list* command:
```bash
./target/release/todo-cli-rust list
```

### list tasks for status

#### status todo

```bash
 ./target/release/todo-cli-rust list todo
```
#### list tasks status done

```bash
 ./target/release/todo-cli-rust list done
```

#### Lists for status in progress

```bash
 ./target/release/todo-cli-rust list inprogress
```


## delete task
delete task,use *delete* command, receive id of task

```bash
 ./target/release/todo-cli-rust delete <id>
```


## Modification status one tasks

### done 
receive id task
```bash
 ./target/release/todo-cli-rust mark-done <id>
```

### todo

```bash
 ./target/release/todo-cli-rust mark-todo <id>
```

### in progress
```bash
 ./target/release/todo-cli-rust mark-in-progress <id>
```









