use druid::{Data, Lens};
use serde::{Serialize, Deserialize};

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub done: bool
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            title: title.to_string(),
            done: false
        }
    }
}