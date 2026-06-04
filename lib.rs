pub mod task;
pub mod worker;
pub mod manager;
pub mod scheduler;
pub mod node;

#[cfg(test)]
mod tests {
    use crate::{task, worker, manager, scheduler, node};
    use uuid::Uuid;
    use std::time::{SystemTime, Instant};

    #[test]
    fn basic() {
        let mut t = task::Task::new(
        Uuid::new_v4(),
        "Task-1",
        task::State::Pending,
        "Image-1",
        1024,
        1);

        let mut te = task::TaskEvent::new(
            Uuid::new_v4(),
            task::State::Pending,
            SystemTime::now(), 
            t.clone()
        ); 

        println!("task: {:?}", t);
        println!("task event: {:?}", te); 
    }
}
