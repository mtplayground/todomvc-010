use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
    pub position: i64,
}

impl Todo {
    pub fn new(id: i64, title: String, completed: bool, position: i64) -> Self {
        Self {
            id,
            title,
            completed,
            position,
        }
    }
}
