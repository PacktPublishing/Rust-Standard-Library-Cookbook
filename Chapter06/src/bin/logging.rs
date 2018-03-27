extern crate env_logger;
#[macro_use]
extern crate log;
use log::Level;

fn main() {
    // env_logger's priority levels are:
    // error > warn > info > debug > trace
    env_logger::init();
    // All logging calls log! in the background
    log!(Level::Debug, "env_logger has been initialized");

    // There are convenience macros for every logging level however
    info!("The program has started!");

    // A log's target is its parent module per default
    // ('logging' in our case, as we're in a binary)
    // We can override this target however:
    info!(target: "extra_info", "This is additional info that will only show if you \
        activate info level logging for the extra_info target");

    warn!("Something that requires your attention happened");

    // Only execute code if logging level is active
    if log_enabled!(Level::Debug) {
        let data = expensive_operation();
        debug!("The expensive operation returned: \"{}\"", data);
    }

    error!("Something terrible happened!");
}

fn expensive_operation() -> String {
    trace!("Starting an expensive operation");
    let data = "Imagine this is a very very expensive task".to_string();
    trace!("Finished the expensive operation");
    data
}
