/*
 * CLI Entry point.
 *
 * Sets up dual diagnostics:
 * 1. miette    — structured terminal rendering for handled errors
 * 2. color-eyre — beautiful panic reports for unhandled crashes
 * 3. tracing   — structured JSON logs to ./logs/api-errors.log
 */

mod errors;

use errors::{CliError, into_lib_report};
use errors_lib::{LibReport, LibResult, ReportExt, handle_error_logic, rootcause::Report};
use miette::NamedSource;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// ---------------------------------------------------------------------------
// App logic — internal functions use Result<_, CliError> for ergonomic `?`
// ---------------------------------------------------------------------------

/// Internal function: uses Result<_, `CliError`> so `?` works on `io::Error`
/// via the From impl SNAFU generated for the Io variant.
fn read_config_file(path: &str) -> Result<String, CliError> {
    let contents = std::fs::read_to_string(path)?; // ← works: From<io::Error> for CliError
    Ok(contents)
}

/// Boundary function: wraps into `LibReport` for the framework pipeline.
fn perform_task() -> LibResult<(), CliError> {
    let err = CliError::ConfigParseError {
        path: "config.json".into(),
        src: NamedSource::new("config.json", "{ \"key\": !!invalid }".to_string()),
        span: (10, 9).into(),
    };

    Err(LibReport(Report::new(err).attach(
        "The application cannot proceed without a valid config.",
    )))
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> miette::Result<()> {
    // 1. Install color-eyre for beautiful panic reports
    color_eyre::install().expect("Failed to install color-eyre");

    // 2. Setup file appender for structured JSON logs
    let file_appender = tracing_appender::rolling::daily("logs", "api-errors.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 3. Respect RUST_LOG or default to 'off'
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("off"));

    tracing_subscriber::registry()
        .with(fmt::layer().json().with_writer(non_blocking))
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .compact()
                .with_filter(filter),
        )
        .init();

    // 4. Miette hook for structured panic diagnostics
    miette::set_panic_hook();

    // ---------------------------------------------------------------------------
    // Demo 1: structured config parse error with source snippet
    // ---------------------------------------------------------------------------
    println!("--- Demo 1: Config parse error ---");
    if let Err(report) = perform_task() {
        handle_error_logic(&report);

        let api_err = report.to_api_error();
        eprintln!("\n[Diagnostic ID: {}]", api_err.correlation_id);

        return Err(miette::Report::new(report));
    }

    // ---------------------------------------------------------------------------
    // Demo 2: `?` working on io::Error inside Result<_, CliError>,
    // then wrapped into LibReport at the boundary via into_lib_report().
    // ---------------------------------------------------------------------------
    println!("\n--- Demo 2: IO error via ? ---");
    if let Err(report) = into_lib_report(read_config_file("nonexistent.json").map(|_| ())) {
        handle_error_logic(&report);

        let api_err = report.to_api_error();
        eprintln!("\n[Diagnostic ID: {}]", api_err.correlation_id);
        eprintln!("IO error caught: {}", api_err.title);
    }

    Ok(())
}
