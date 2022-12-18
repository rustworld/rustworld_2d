use gotham::state::State;


pub fn say_hello(state:State) -> (State, &'static str) {
    (state, "Hello Gotham")
}
