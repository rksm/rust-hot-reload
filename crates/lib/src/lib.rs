#[derive(Default, Debug)]
pub struct State {
    pub called: usize,
}

#[no_mangle]
pub extern "C" fn step(state: &mut State) {
    state.called += 1;
}
