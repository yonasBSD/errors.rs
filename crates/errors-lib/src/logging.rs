pub fn init_logging() {
    use tracing_error::ErrorLayer;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();
}

pub fn init_env() {
    unsafe {
        std::env::set_var("RUST_LIB_BACKTRACE", "1");
        std::env::set_var("RUST_BACKTRACE", "full");
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("ROOTCAUSE_BACKTRACE", "full_paths");
    }
}
