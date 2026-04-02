## Acknowledgements

This project is based on a challenge from roadmap.sh:
[ fileTask Tracker Challenge - roadmap.sh](https://roadmap.sh/projects/task-tracker)


```bash
https://github.com/AppBlitz/todo-cli-rust.git
```


```cargo
#Create new task
cargo run -- add "<description task>"

# update description task for id
cargo run -- update <id_task> "<description task>"

# delete task for id
cargo run -- delete <id_task>

# show all tasks created
cargo run -- list

# task in progress
cargo run -- mark-in-progress "<id_task>"

# task in done
cargo run -- mark-done "<id task>"

# show all list from status is done
cargo run -- list done

# show all list from status is todo
cargo run -- list todo

# show all list from status is in-progress
cargo run -- in-progress
```

All implementation and source code in this repository were written by me.
