use std::collections::{HashMap};
use std::vec::Vec;
use uuid::{Uuid}; 
use crate::task::Task;

struct Worker<T> {
    name:       String,
    queue:      Vec<T>,
    db:         HashMap<Uuid, Task>,
    task_count: usize,
}

impl<T> Worker<T> {
    pub fn collect_stats(&self) {
        println!("Collecting stats..");
    }

    pub fn run_task(&self) {
        println!("running/stopping this task..");
    }

    pub fn start_task(&self) {
        println!("start this task..");
    }

    pub fn stop_task(&self) {
        println!("stop this task..");
    }
}
