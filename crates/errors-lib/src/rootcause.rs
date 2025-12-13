use std::fs;
use rootcause::{prelude::*, hooks::Hooks};
use rootcause_backtrace::BacktraceCollector;
use tracing::instrument;

#[derive(Debug, Clone)]
enum ParseError {
    InvalidFormat,
    MissingField(String),
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "Invalid format"),
            Self::MissingField(field) => write!(f, "Missing required field: {field}"),
        }
    }
}

impl core::error::Error for ParseError {}

// Report<ParseError> accepts both ParseError and Report<ParseError>
fn parse_value(input: &str) -> Result<u32, Report<ParseError>> {
    if input.is_empty() {
        // Raw ParseError → ? converts to Report<ParseError>
        Err(ParseError::InvalidFormat)?;
    }

    if !input.chars().all(|c| c.is_ascii_digit()) {
        // Report<ParseError> → ? passes through
        Err(report!(ParseError::InvalidFormat).attach(format!("Input: {input}")))?;
    }

    let value: u32 = input
        .parse()
        .context(ParseError::MissingField("value".to_string()))?;

    Ok(value)
}

// Report (dynamic) accepts any error type
fn process_file(path: &str) -> Result<u32, Report> {
    // io::Error → Report
    let contents = fs::read_to_string(path).attach(format!("Path: {path}"))?;

    // Report<ParseError> → Report
    let value = parse_value(&contents)?;

    Ok(value)
}

#[instrument(level = "debug", target = "errors::rootcause", name = "run")]
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Capture backtraces for all errors
    Hooks::new()
        .report_creation_hook(BacktraceCollector::new_from_env())
        .install()
        .expect("failed to install hooks");

    println!("Typed errors - Report<ParseError>:\n");

    /*
    if let Err(report) = parse_value("") {
        eprintln!("{report}");
    }

    if let Err(report) = parse_value("abc") {
        eprintln!("{report}");
    }
    */

    println!("Dynamic errors - mixing types:\n");
    if let Err(report) = process_file("/nonexistent/config.txt") {
        //eprintln!("{report}");
        use error_stack::Report as StackReport;
        use color_eyre::Report as EyreReport;
        use std::fs::File;
        use std::io::Write;
        use rootcause::{
            bail,
            compat::{IntoRootcause, error_stack06::IntoErrorStack, eyre06::IntoEyre},
        };

        // Convert into error-stack
        let stack_report: StackReport<_> = report.into_error_stack();
        eprintln!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");
        eprintln!("{stack_report:?}");
        eprintln!(" ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^");

        // Convert into color-eyre
        /*
        let eyre_report: EyreReport = report.into_eyre();
        let mut file = File::create("/tmp/error.log")?;
        writeln!(file, "{eyre_report:?}")?;
        */
    }

    Ok(())
}
