use lib::*;

#[cfg(feature = "reload")]
hot_lib_reloader::define_lib_reloader! {
    unsafe MyLibLoader {
        lib_name: "lib",
        source_files: ["../lib/src/lib.rs"]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::default();

    #[cfg(feature = "reload")]
    let mut lib = MyLibLoader::new()?;

    loop {
        #[cfg(feature = "reload")]
        lib.update()?;

        #[cfg(feature = "reload")]
        lib.step(&mut state);

        #[cfg(not(feature = "reload"))]
        lib::step(&mut state);

        dbg!(&state);

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
