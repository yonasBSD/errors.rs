pub mod logging;
pub mod color_eyre;
pub mod error_stack;
pub mod rootcause;

/*
//! Unified error abstraction over error-stack, rootcause, and color-eyre.
//!
//! Public surface:
//! - `Result<T>`
//! - `Error`
//! - `init()` to install pretty reports (color-eyre if enabled)
//! - `WithContext` to attach context uniformly

use std::fmt;
use std::error::Error as StdError;

pub type Result<T> = std::result::Result<T, Error>;

/// Install a global reporter if available (color-eyre).
/// Safe to call multiple times; only installs once.
pub fn init() {
    // color-eyre installs a global hook for pretty reports
    let _ = color_eyre::install();
}

/// A unified error that can wrap different backend error types.
/// You only depend on this `Error` in your application.
#[derive(Debug)]
pub struct Error {
    inner: Inner,
    context: Vec<String>,
}

#[derive(Debug)]
enum Inner {
    Std(Box<dyn StdError + Send + Sync>),

    Eyre(eyre::Report),

    ErrorStack(error_stack::Report),

    RootCause(rootcause::Error), // rootcause::Error implements std::error::Error
}

impl Error {
    /// Create from any standard error.
    pub fn new<E>(err: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            inner: Inner::Std(Box::new(err)),
            context: Vec::new(),
        }
    }

    /// Attach human-readable context.
    pub fn with_context<S: Into<String>>(mut self, msg: S) -> Self {
        self.context.push(msg.into());
        self
    }

    /// Attach human-readable context, by reference.
    pub fn push_context<S: Into<String>>(&mut self, msg: S) {
        self.context.push(msg.into());
    }

    /// Access attached context frames.
    pub fn context(&self) -> &[String] {
        &self.context
    }

    /// Format a detailed report suitable for user output.
    pub fn report(&self) -> String {
        let mut out = String::new();

        // Heading and context frames
        if !self.context.is_empty() {
            out.push_str("Context:\n");
            for (i, c) in self.context.iter().enumerate() {
                out.push_str(&format!("  {i}: {c}\n"));
            }
            out.push('\n');
        }

        // Backend-specific pretty display
        match &self.inner {
            Inner::Std(e) => {
                out.push_str(&format!("Error: {e}\n"));
                // Try source chain
                let mut cur = e.source();
                let mut idx = 0;
                while let Some(src) = cur {
                    out.push_str(&format!("  caused by ({idx}): {src}\n"));
                    cur = src.source();
                    idx += 1;
                }
            }

            Inner::Eyre(rep) => {
                // eyre::Report already pretty-prints with backtrace if available
                out.push_str(&format!("{rep}\n"));
            }

            Inner::ErrorStack(rep) => {
                // error-stack::Report includes frames/contexts
                out.push_str(&format!("{rep}\n"));
            }

            Inner::RootCause(rc) => {
                // rootcause::Error implements Display with cause tree
                out.push_str(&format!("{rc}\n"));
            }
        }

        out
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Short display: last context + top-level message
        if let Some(last) = self.context.last() {
            write!(f, "{last}: ")?;
        }
        match &self.inner {
            Inner::Std(e) => fmt::Display::fmt(e, f),

            Inner::Eyre(rep) => fmt::Display::fmt(rep, f),

            Inner::ErrorStack(rep) => fmt::Display::fmt(rep, f),

            Inner::RootCause(rc) => fmt::Display::fmt(rc, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.inner {
            Inner::Std(e) => e.source(),

            Inner::Eyre(rep) => rep.source(),

            Inner::ErrorStack(rep) => rep.source(),

            Inner::RootCause(rc) => rc.source(),
        }
    }
}

// -------- Conversions from common backends --------

impl From<eyre::Report> for Error {
    fn from(rep: eyre::Report) -> Self {
        Self { inner: Inner::Eyre(rep), context: Vec::new() }
    }
}

impl From<eyre::ErrReport> for Error {
    fn from(rep: eyre::ErrReport) -> Self {
        // eyre::ErrReport is just a Display wrapper; convert via Report
        let rep: eyre::Report = rep.into();
        Self { inner: Inner::Eyre(rep), context: Vec::new() }
    }
}

impl From<error_stack::Report> for Error {
    fn from(rep: error_stack::Report) -> Self {
        Self { inner: Inner::ErrorStack(rep), context: Vec::new() }
    }
}

impl From<rootcause::Error> for Error {
    fn from(rc: rootcause::Error) -> Self {
        Self { inner: Inner::RootCause(rc), context: Vec::new() }
    }
}

/// Attach context uniformly to `Result<T>`.
pub trait WithContext<T> {
    fn with_ctx<S: Into<String>>(self, msg: S) -> Result<T>;
}

impl<T, E> WithContext<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn with_ctx<S: Into<String>>(self, msg: S) -> Result<T> {
        self.map_err(|e| e.into().with_context(msg))
    }
}

// Convenience constructors for common patterns.

/// Construct an `Error` from a string message.
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::new(SimpleMsg(s))
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error::new(SimpleMsg(s.to_string()))
    }
}

#[derive(Debug)]
struct SimpleMsg(String);

impl fmt::Display for SimpleMsg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl StdError for SimpleMsg {}
*/
