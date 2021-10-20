use serde_derive::*;
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoAction {
    AddTask(String),
    DeleteTask(usize),
    CompleteTask(usize),
    ReopenTask(usize),
}