mod cores;

use tracing::debug;
use crate::cores::logging::init::init_tracer;

pub fn run() -> Result<(), String> {
    let _ = init_tracer();
    debug!("hello world...");

    Ok(())
}