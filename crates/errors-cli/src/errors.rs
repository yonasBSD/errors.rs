/*
 * CLI-specific error definitions.
 *
 * Pattern for consuming errors-lib:
 * 1. Define your own error enum with snafu + miette.
 * 2. Use #[snafu(context(false))] to get From<io::Error> for CliError — this
 *    makes `?` work inside functions returning Result<_, CliError>.
 * 3. At the boundary (main, top-level handlers), wrap CliError in LibReport.
 *
 * Why not use LibResult<_, CliError> everywhere:
 * Rust's orphan rule prevents implementing From<io::Error> for
 * LibReport<CliError> in this crate, because neither io::Error nor LibReport
 * is defined here. The solution is to use Result<_, CliError> for internal
 * functions and only wrap into LibReport at the top-level boundary.
 */

use miette::{Diagnostic, NamedSource, SourceSpan};
use snafu::prelude::*;

#[derive(Debug, Snafu, Diagnostic)]
#[snafu(visibility(pub))]
pub enum CliError {
    /// Config file could not be parsed — includes source snippet rendering.
    #[snafu(display("Failed to parse config at {path}"))]
    #[diagnostic(
        code(config::invalid_format),
        help("Ensure the configuration file is valid JSON.")
    )]
    ConfigParseError {
        path: String,
        /// The source text, used by miette/Ariadne for snippet rendering.
        #[source_code]
        src: NamedSource<String>,
        /// Points miette at the exact location of the syntax error.
        #[label("syntax error here")]
        span: SourceSpan,
    },

    /// Network call timed out.
    #[snafu(display("Network timeout after {timeout}s"))]
    #[diagnostic(
        code(network::timeout),
        help("Check network connectivity and consider increasing the timeout.")
    )]
    NetworkTimeout { timeout: u64 },

    /// Wraps std::io::Error.
    ///
    /// #[snafu(context(false))] generates: From<std::io::Error> for CliError.
    /// This makes `?` work in functions returning Result<_, CliError>.
    #[snafu(context(false))]
    #[snafu(display("IO error: {source}"))]
    #[diagnostic(code(io::error))]
    Io { source: std::io::Error },
}

/// Helper to wrap a CliError result into a LibReport at the boundary.
pub fn into_lib_report(r: Result<(), CliError>) -> errors_lib::LibResult<(), CliError> {
    r.map_err(|e| errors_lib::LibReport(errors_lib::rootcause::Report::new(e)))
}
