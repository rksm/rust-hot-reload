use lib::State;
use std::error::Error;

#[cfg(feature = "reload")]
mod lib_reloader;

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "reload")]
    let mut lib =
        lib_reloader::LibReloader::new("target/debug", "liblib").expect("initial load the lib");

    let mut state = State::default();

    loop {
        #[cfg(feature = "reload")]
        step(&mut lib, &mut state)?;

        #[cfg(not(feature = "reload"))]
        step(&mut state)?;

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(feature = "reload")]
fn step(
    lib: &mut lib_reloader::LibReloader,
    state: &mut State,
) -> Result<(), Box<dyn Error>> {
    lib.update()?;
    unsafe {
        lib.with_symbol_do(b"step", |sym: libloading::Symbol<fn(arg: &mut State)>| { sym(state) })?;
    }

    Ok(())
}

#[cfg(not(feature = "reload"))]
fn step(state: &mut State) -> Result<(), Box<dyn Error>> {
    lib::step(state);
    Ok(())
}
