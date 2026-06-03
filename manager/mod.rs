use std::vec::{Vec};
use std::collections::{HashMap};
use uuid::{Uuid}; 
use crate::task::Task;

pub struct Manager<T> {
    pending:            Vec<T>,
    task_db:            HashMap<String, Task>,
    event_db:           HashMap<String, Task>,
    workers:            Vec<String>,
    workers_task_map:   HashMap<String, Uuid>,
    task_workers_map:   HashMap<Uuid, String>
}

impl<T> Manager {
    pub fn select_worker(&self) {
        todo!("");
    }

    pub fn update_worker(&self) {
        todo!("");
    }

    pub fn send_work(&self) {
        todo!("");
    }
}
