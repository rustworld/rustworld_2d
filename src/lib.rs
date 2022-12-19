use gotham::state::State;
use std::sync::atomic::{AtomicUsize, Ordering};


static TOTAL_CALLS: AtomicUsize = AtomicUsize::new(0);

pub fn say_hello(state:State) -> (State, String) {
    let calls = TOTAL_CALLS.fetch_add(1, Ordering::SeqCst) + 1;
    let response = format!("Hello Gotham: {}", calls);
    (state, response)
}
