use crate::models::model_task::Todo;

pub fn create_id_task(tasks_vector: &Vec<Todo>, auxiliary_vector_tasks: u64) -> usize {
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
