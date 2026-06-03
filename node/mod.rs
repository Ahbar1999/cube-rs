use std::net::{IpAddr}; 

struct Node {
    name:               String,
    ip:                 IpAddr,
    cores:              usize,
    memory:             usize,
    memory_allocated:   usize,
    disk:               usize,
    disk_allocated:     usize,
    role:               String,
    task_count:         usize,
}
