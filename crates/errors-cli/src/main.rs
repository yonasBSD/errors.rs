use errors_lib::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging();

    /*
    tracing::info!(method = "error stack", "Begin");
    let _ = error_stack::run();

    logging::init_env();

    tracing::info!(method = "root cause", "Begin");
    rootcause::run();

    tracing::info!(method = "color eyre", "Begin");
    let _ = color_eyre::run();
    */

    rootcause::run();

    human_panic::run();

    Ok(())
}
