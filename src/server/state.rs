pub struct State {}

unsafe impl Send for State {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
}
