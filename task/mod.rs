use uuid::{Uuid};
use std::marker::PhantomData;
use std::collections::{HashMap};
use std::time::{SystemTime};

// should zero sized marker types be used to state ??
// I chose enums because there's too much common functionality between states and there
// are a lot of other places whre states are being used to denote type rather than just transitions
enum State {
    PendingState,
    Scheduled,
    Runnin,
    Completed,
    Failed,
}

pub struct Task {
    id:             Uuid,
    name:           String,
    state:          State,
    image:          String,
    memory:         usize,
    disk:           usize,
    exposed_ports:  HashMap<String, u16>,
    port_bindings:  HashMap<String, String>,
    restart_policy: String,
    start_time:     SystemTime,
    finish_time:    SystemTime,
} 

struct TaskEvent<State> {
    id:         Uuid,
    state:      State,
    timestamp:  SystemTime,
    task:       Task,
} 

