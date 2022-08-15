#[cfg(feature = "reload")]
use hot_lib::*;
#[cfg(not(feature = "reload"))]
use lib::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    pub use lib::State;

    // get all public #[no_mangle] functions from that file and generate
    // functions with the same signatures that are hot-reloadable.
    hot_functions_from_file!("lib/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}

    // a monotonically increasing counter (starting with 0) that counts library reloads
    #[lib_version]
    pub fn version() -> usize {}
}

fn main() {
    let mut state = State { counter: 0 };
    loop {
        step(&mut state);
        dbg!(&state);

        println!("waiting for library change...");
        // Wait until a library change happens (but the old lib is still loader)
        let token = hot_lib::subscribe().wait_for_about_to_reload();
        // while token exists, reload is blocked
        drop(token);

        // wait for reload to be done
        hot_lib::subscribe().wait_for_reload();
        println!("... library has been reloaded {} times", hot_lib::version());
    }
}
