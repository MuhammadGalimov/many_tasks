use std::convert::TryInto;
use std::rc::Rc;
use druid::{Data, Lens, im};

use crate::task::Task;
use crate::project::Project;

type TaskId = u64;
type ProjectId = u64;

#[derive(Clone, Data, Lens)]
pub struct State {
    tasks: im::HashMap<TaskId, Rc<Task>>,
    projects: im::HashMap<ProjectId, Project>
}

impl State {
    pub fn new() -> Self {
        let tasks: im::HashMap<TaskId, Rc<Task>> = im::HashMap::new();
        let projects: im::HashMap<ProjectId, Project> = im::HashMap::new();

        State { tasks, projects }
    }

    pub fn save(&self) {
        let mut out = String::from("");

        for (_, &task) in self.tasks.iter() {
            // todo: вот тут подумать!
            out.push_str(&serde_json::to_string(Rc::into_raw(task)).unwrap()[..]);
        }
    }
}