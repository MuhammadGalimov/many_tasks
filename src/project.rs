use druid::{im, Data, Lens};
use serde::{Serialize, Deserialize};

use crate::task::Task;

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct Project {
    tasks: im::Vector<Task>
}