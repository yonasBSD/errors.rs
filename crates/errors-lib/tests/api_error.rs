/*
 * Integration tests for ApiError serialization.
 *
 * Tests library primitives directly — no dependency on errors-cli's
 * perform_task(). We build a LibReport inline using a minimal test
 * error type, keeping errors-lib self-contained.
 */

use errors_lib::{LibReport, ReportExt, rootcause::Report};
use miette::{Diagnostic, NamedSource, SourceSpan};
use serde_json::Value;
use snafu::prelude::*;

// ---------------------------------------------------------------------------
// Minimal error type for testing — mirrors what a consuming crate would define
// ---------------------------------------------------------------------------

#[derive(Debug, Snafu, Diagnostic)]
enum TestError {
    #[snafu(display("Failed to parse config at {path}"))]
    #[diagnostic(
        code(config::invalid_format),
        help("Ensure the configuration file is valid JSON.")
    )]
    ConfigParseError {
        path: String,
        #[source_code]
        src: NamedSource<String>,
        #[label("syntax error here")]
        span: SourceSpan,
    },
}

fn make_report() -> LibReport<TestError> {
    let err = TestError::ConfigParseError {
        path: "config.json".into(),
        src: NamedSource::new("config.json", "{ \"key\": !!invalid }".to_string()),
        span: (10, 9).into(),
    };
    LibReport(Report::new(err).attach("The application cannot proceed without a valid config."))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[test]
fn test_api_error_json_structure() {
    let api_error = make_report().to_api_error();

    let json_value =
        serde_json::to_value(&api_error).expect("Failed to serialize ApiError to JSON");

    assert_eq!(json_value["code"], "config::invalid_format");
    assert!(
        json_value["title"]
            .as_str()
            .unwrap()
            .contains("Failed to parse config")
    );

    // correlation_id exists and is the correct length (nanoid!(8))
    let id = json_value["correlation_id"]
        .as_str()
        .expect("correlation_id missing");
    assert_eq!(id.len(), 8);

    // git_hash is present
    assert!(json_value.get("git_hash").is_some());

    // history contains the attachment
    let history = json_value["history"].as_array().expect("history missing");
    assert!(
        history
            .iter()
            .any(|h| h.as_str().unwrap().contains("valid config"))
    );
}

#[test]
fn test_snapshot_api_error() {
    let api_error = make_report().to_api_error();

    // Redact volatile fields before snapshotting
    let mut redacted = serde_json::to_value(&api_error).unwrap();
    redacted["correlation_id"] = Value::String("REDACTED_ID".to_string());
    redacted["git_hash"] = Value::String("REDACTED_HASH".to_string());

    insta::assert_json_snapshot!(redacted);
}
