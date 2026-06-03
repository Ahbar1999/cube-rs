use uuid::{Uuid};
use std::marker::PhantomData;

// zero sized marker types; 
// invalid transitions are ruled out at compile time!
struct PendingState;
struct Scheduled;
struct Running;
struct Completed;
struct Failed;

struct Task<State> {
    id:     Uuid,
    name:   String,
    state:  PhantomData<State>
} 


