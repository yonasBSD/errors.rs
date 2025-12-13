/*
 * Core library: framework machinery only.
 *
 * This crate provides:
 * 1. LibReport   — a miette-compatible wrapper around rootcause::Report<E>
 * 2. LibResult   — a Result alias using LibReport as the error type
 * 3. ApiError    — machine-readable error struct for API/log sinks
 * 4. ReportExt   — trait to convert a LibReport into an ApiError
 * 5. handle_error_logic — example of typed introspection via rootcause
 *
 * Consuming crates define their own error enums (with snafu + miette),
 * then wrap them in LibReport<YourError> for full framework integration.
 *
 * Uses:
 *   rootcause : typed error chain with introspection
 *   miette    : structured diagnostics and terminal rendering
 *   snafu     : ergonomic error definition (used by consumers, re-exported)
 *   tracing   : structured log emission on error
 *   nanoid    : correlation ID generation
 */

use std::fmt;

use miette::{Diagnostic, SourceCode};
use nanoid::nanoid;
pub use rootcause;
use rootcause::Report;
use serde::{Serialize, Serializer};
pub use snafu;
use tracing::error;

// ---------------------------------------------------------------------------
// Core types
// ---------------------------------------------------------------------------

/// A miette-compatible wrapper around a rootcause error chain.
///
/// `E` is the top-level error context type — defined by the consuming crate,
/// not by this library. It must implement `Diagnostic` (for miette rendering)
/// and `std::error::Error`.
#[derive(Debug)]
pub struct LibReport<E>(pub Report<E>)
where
    E: Diagnostic + fmt::Display + fmt::Debug + Send + Sync + 'static;

/// Result alias. Consuming crates alias this with their own error type:
///
/// ```rust
/// type AppResult<T> = errors_lib::LibResult<T, AppError>;
/// ```
pub type LibResult<T, E> = std::result::Result<T, LibReport<E>>;

// ---------------------------------------------------------------------------
// API / log sink types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct ErrorFrame {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub git_hash: String,
    pub docs_url: String,
    pub correlation_id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(serialize_with = "serialize_history_flat")]
    pub history: Vec<ErrorFrame>,
}

fn serialize_history_flat<S>(history: &[ErrorFrame], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let flat: Vec<&str> = history.iter().map(|f| f.message.as_str()).collect();
    flat.serialize(serializer)
}

// ---------------------------------------------------------------------------
// Diagnostic impl — delegates to the inner error context
// ---------------------------------------------------------------------------

impl<E> Diagnostic for LibReport<E>
where
    E: Diagnostic + fmt::Display + fmt::Debug + Send + Sync + 'static,
{
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.0.current_context().code()
    }

    fn severity(&self) -> Option<miette::Severity> {
        self.0.current_context().severity()
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.0.current_context().help()
    }

    /// Maps the error code to a clickable docs link in the terminal.
    fn url<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        let base = env!("ERROR_DOCS_URL");
        self.code().map(|c| {
            let link = format!("{}/#{}", base, c);
            Box::new(link) as Box<dyn fmt::Display>
        })
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.0.current_context().source_code()
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = miette::LabeledSpan> + '_>> {
        self.0.current_context().labels()
    }
}

impl<E> fmt::Display for LibReport<E>
where
    E: Diagnostic + fmt::Display + fmt::Debug + Send + Sync + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<E> std::error::Error for LibReport<E> where
    E: Diagnostic + fmt::Display + fmt::Debug + Send + Sync + 'static
{
}

// ---------------------------------------------------------------------------
// ReportExt — converts a LibReport into an ApiError for logging/API sinks
// ---------------------------------------------------------------------------

pub trait ReportExt {
    fn to_api_error(&self) -> ApiError;
}

impl<E> ReportExt for LibReport<E>
where
    E: Diagnostic + fmt::Display + fmt::Debug + Send + Sync + 'static,
{
    fn to_api_error(&self) -> ApiError {
        let mut history = Vec::new();
        for node in self.0.iter_reports() {
            for attachment in node.attachments().iter() {
                history.push(ErrorFrame {
                    message: attachment.to_string(),
                });
            }
        }

        let ctx = self.0.current_context();
        let api_err = ApiError {
            git_hash: env!("GIT_HASH").to_string(),
            docs_url: env!("ERROR_DOCS_URL").to_string(),
            correlation_id: nanoid!(8),
            title: ctx.to_string(),
            code: ctx.code().map(|c| c.to_string()),
            help: ctx.help().map(|h| h.to_string()),
            history,
        };

        error!(
            hash = %api_err.git_hash,
            docs = %api_err.docs_url,
            id = %api_err.correlation_id,
            title = %api_err.title,
            code = api_err.code.as_deref(),
            history = ?api_err.history.iter().map(|h| &h.message).collect::<Vec<_>>(),
            "Internal error reported to API sink"
        );

        api_err
    }
}

// ---------------------------------------------------------------------------
// handle_error_logic — example typed introspection via rootcause
// ---------------------------------------------------------------------------

/// Walk the error chain and react to specific error types.
/// This is the pattern for "smart" error handling — not just logging,
/// but branching on what actually went wrong.
pub fn handle_error_logic<E>(report: &LibReport<E>)
where
    E: Diagnostic + fmt::Display + fmt::Debug + Send + Sync + 'static,
{
    for node in report.0.iter_reports() {
        if let Some(io_err) = node.downcast_current_context::<std::io::Error>() {
            if matches!(io_err.kind(), std::io::ErrorKind::NotFound) {
                println!("--- LOGIC CHECK: Missing file detected ---");
            }
        }
    }
}
