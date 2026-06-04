use uuid::{Uuid};
use std::marker::PhantomData;
use std::collections::{HashMap};
use std::time::{SystemTime};

// should zero sized marker types be used to state ??
// I chose enums because there's too much common functionality between states and there
// are a lot of other places whre states are being used to denote type rather than just transitions
#[derive(Clone, Debug)]
pub enum State {
    Pending,
    Scheduled,
    Running,
    Completed,
    Failed,
}

#[derive(Clone, Debug)]
pub struct Task<'this> {
    pub id:             Uuid,
    pub name:           &'this str, // lifetime of 'this' task 
    pub state:          State,
    pub image:          &'this str,
    pub memory:         usize,
    pub disk:           usize,
    pub exposed_ports:  HashMap<String, u16>,
    pub port_bindings:  HashMap<String, String>,
    pub restart_policy: Option<String>,
    pub start_time:     Option<SystemTime>,
    pub finish_time:    Option<SystemTime>,
} 

impl<'this> Task<'this> {
    pub fn new(id: Uuid, name: &'this str, state: State, image: &'this str, memory: usize, disk: usize) -> Self {
        Task {
            id,
            name,
            state, 
            image, 
            memory, 
            disk,
            exposed_ports: HashMap::new(),
            port_bindings: HashMap::new(),
            restart_policy: None,
            start_time:    None,
            finish_time: None,
        }
    } 
}

#[derive(Debug, Clone)]
pub struct TaskEvent<'this, State> {
    id:         Uuid,
    state:      State,
    timestamp:  SystemTime,
    task:       Task<'this>,
} 

impl<'this, State> TaskEvent<'this, State> {
    pub fn new(id: Uuid, state: State, timestamp: SystemTime, task: Task<'this>) -> Self {
        Self {
            id, 
            state, 
            timestamp,
            task
        } 
    } 
}

