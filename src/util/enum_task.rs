use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum StatusTaks {
    Todo,
    InProgress,
    Done,
}
