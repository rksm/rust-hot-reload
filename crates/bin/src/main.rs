use lib::State;
use std::error::Error;

#[cfg(feature = "reload")]
hot_lib_reloader::define_lib_reloader!(
    MyLibLoader("target/debug", "lib") {
        fn step(arg: &mut State) -> ();
    }
);

fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "reload")]
    let mut lib = MyLibLoader::new().expect("init lib loader");

    let mut state = State::default();

    loop {
        #[cfg(feature = "reload")]
        lib.update().expect("lib update");

        #[cfg(feature = "reload")]
        lib.step(&mut state)?;

        #[cfg(not(feature = "reload"))]
        lib::step(state);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
