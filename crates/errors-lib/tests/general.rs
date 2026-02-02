/*
 * Comprehensive Test Suite for errors-lib
 * 
 * Test Categories:
 * 1. Error Construction & Display
 * 2. Diagnostic Trait Implementation
 * 3. Report Extension Traits
 * 4. Error Tree Navigation
 * 5. Serialization & API Error Format
 * 6. Integration Tests
 * 7. Snapshot Tests
 */

#[cfg(test)]
mod error_construction {
    use errors_lib::types::LibError;
    use miette::{Diagnostic, NamedSource};

    #[test]
    fn test_config_parse_error_display() {
        let err = LibError::ConfigParseError {
            path: "test.json".to_string(),
            src: NamedSource::new("test.json", r#"{"key": invalid}"#.to_string()),
            span: (8, 7).into(),
        };

        let display = format!("{}", err);
        assert!(display.contains("Failed to parse config at test.json"));
    }

    #[test]
    fn test_network_error_display() {
        let err = LibError::NetworkError { timeout: 30 };
        let display = format!("{}", err);
        assert_eq!(display, "Network timeout after 30s");
    }

    #[test]
    fn test_config_error_has_diagnostic_code() {
        let err = LibError::ConfigParseError {
            path: "config.json".to_string(),
            src: NamedSource::new("config.json", "{}".to_string()),
            span: (0, 1).into(),
        };

        let code = LibError::code(&err);
        assert!(code.is_some());
        assert_eq!(code.unwrap().to_string(), "config::invalid_format");
    }

    #[test]
    fn test_config_error_has_help_text() {
        let err = LibError::ConfigParseError {
            path: "config.json".to_string(),
            src: NamedSource::new("config.json", "{}".to_string()),
            span: (0, 1).into(),
        };

        let help = LibError::help(&err);
        assert!(help.is_some());
        assert_eq!(
            help.unwrap().to_string(),
            "Ensure the configuration file is valid JSON."
        );
    }

    #[test]
    fn test_error_source_code_available() {
        let source = r#"{"key": !!invalid}"#;
        let err = LibError::ConfigParseError {
            path: "test.json".to_string(),
            src: NamedSource::new("test.json", source.to_string()),
            span: (8, 9).into(),
        };

        let src_code = LibError::source_code(&err);
        assert!(src_code.is_some());
    }

    #[test]
    fn test_error_has_labels() {
        let err = LibError::ConfigParseError {
            path: "test.json".to_string(),
            src: NamedSource::new("test.json", "invalid".to_string()),
            span: (0, 7).into(),
        };

        let labels: Vec<_> = LibError::labels(&err).unwrap().collect();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].label().unwrap(), "syntax error here");
    }
}

#[cfg(test)]
mod report_wrapper {
    use errors_lib::{types::LibError, LibReport};
    use miette::{Diagnostic, NamedSource};
    use rootcause::Report;

    #[test]
    fn test_lib_report_wraps_rootcause_report() {
        let err = LibError::NetworkError { timeout: 10 };
        let report = Report::new(err);
        let lib_report = LibReport(report);

        assert!(format!("{}", lib_report).contains("Network timeout"));
    }

    #[test]
    fn test_lib_report_implements_diagnostic() {
        let err = LibError::ConfigParseError {
            path: "app.json".to_string(),
            src: NamedSource::new("app.json", "{}".to_string()),
            span: (0, 1).into(),
        };
        let lib_report = LibReport(Report::new(err));

        assert!(lib_report.code().is_some());
        assert!(lib_report.help().is_some());
        assert!(lib_report.source_code().is_some());
    }

    #[test]
    fn test_lib_report_url_generation() {
        let err = LibError::ConfigParseError {
            path: "config.json".to_string(),
            src: NamedSource::new("config.json", "{}".to_string()),
            span: (0, 1).into(),
        };
        let lib_report = LibReport(Report::new(err));

        let url = lib_report.url();
        assert!(url.is_some());
        let url_str = url.unwrap().to_string();
        assert!(url_str.contains("docs.rs/errors-lib"));
        assert!(url_str.contains("#config::invalid_format"));
    }

    #[test]
    fn test_lib_report_implements_error_trait() {
        let err = LibError::NetworkError { timeout: 5 };
        let lib_report = LibReport(Report::new(err));

        // Should implement std::error::Error
        let _: &dyn std::error::Error = &lib_report;
    }
}

#[cfg(test)]
mod report_extension_trait {
    use errors_lib::{types::LibError, LibReport, ReportExt};
    use miette::NamedSource;
    use rootcause::Report;

    #[test]
    fn test_to_api_error_basic() {
        let err = LibError::NetworkError { timeout: 30 };
        let lib_report = LibReport(Report::new(err));

        let api_err = lib_report.to_api_error();

        assert_eq!(api_err.title, "Network timeout after 30s");
        assert!(api_err.code.is_none()); // NetworkError doesn't have a code
        assert!(api_err.correlation_id.len() == 8); // nanoid(8)
    }

    #[test]
    fn test_to_api_error_with_metadata() {
        let err = LibError::ConfigParseError {
            path: "settings.json".to_string(),
            src: NamedSource::new("settings.json", "{}".to_string()),
            span: (0, 1).into(),
        };
        let lib_report = LibReport(Report::new(err));

        let api_err = lib_report.to_api_error();

        assert!(api_err.title.contains("Failed to parse config"));
        assert_eq!(api_err.code, Some("config::invalid_format".to_string()));
        assert_eq!(
            api_err.help,
            Some("Ensure the configuration file is valid JSON.".to_string())
        );
    }

    #[test]
    fn test_api_error_contains_git_hash() {
        let err = LibError::NetworkError { timeout: 15 };
        let lib_report = LibReport(Report::new(err));

        let api_err = lib_report.to_api_error();

        assert!(!api_err.git_hash.is_empty());
    }

    #[test]
    fn test_api_error_contains_docs_url() {
        let err = LibError::NetworkError { timeout: 20 };
        let lib_report = LibReport(Report::new(err));

        let api_err = lib_report.to_api_error();

        assert!(api_err.docs_url.contains("docs.rs/errors-lib"));
    }

    #[test]
    fn test_api_error_with_attachments() {
        let err = LibError::ConfigParseError {
            path: "db.json".to_string(),
            src: NamedSource::new("db.json", "malformed".to_string()),
            span: (0, 9).into(),
        };
        let report = Report::new(err)
            .attach("Database initialization failed")
            .attach("Fallback to defaults not available");
        let lib_report = LibReport(report);

        let api_err = lib_report.to_api_error();

        assert_eq!(api_err.history.len(), 2);
        assert_eq!(api_err.history[0].message, "Database initialization failed");
        assert_eq!(
            api_err.history[1].message,
            "Fallback to defaults not available"
        );
    }
}

#[cfg(test)]
mod error_tree_navigation {
    use errors_lib::{handle_error_logic, types::LibError, LibReport};
    use miette::NamedSource;
    use rootcause::Report;

    #[test]
    fn test_tree_iteration_single_error() {
        let err = LibError::NetworkError { timeout: 10 };
        let lib_report = LibReport(Report::new(err));

        let count = lib_report.0.iter_reports().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_tree_iteration_with_children() {
        let err1 = LibError::NetworkError { timeout: 5 };
        let err2 = LibError::ConfigParseError {
            path: "net.json".to_string(),
            src: NamedSource::new("net.json", "{}".to_string()),
            span: (0, 1).into(),
        };

        let report = Report::new(err2).with_child(Report::new(err1));
        let lib_report = LibReport(report);

        let count = lib_report.0.iter_reports().count();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_handle_error_logic_with_io_error() {
        // This test verifies that handle_error_logic can detect IO errors
        // when they're part of the error tree
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let report = Report::new(io_err);
        let lib_report = LibReport(report.into_dynamic());

        // Should not panic
        handle_error_logic(&lib_report);
    }

    #[test]
    fn test_downcast_to_specific_error_type() {
        let err = LibError::NetworkError { timeout: 25 };
        let report = Report::new(err);

        // Verify we can downcast to LibError
        let downcasted = report.downcast_report::<LibError>();
        assert!(downcasted.is_ok());
    }
}

#[cfg(test)]
mod serialization {
    use errors_lib::{types::LibError, LibReport, ReportExt};
    use miette::NamedSource;
    use rootcause::Report;
    use serde_json;

    #[test]
    fn test_api_error_serializes_to_json() {
        let err = LibError::NetworkError { timeout: 10 };
        let lib_report = LibReport(Report::new(err));
        let api_err = lib_report.to_api_error();

        let json = serde_json::to_value(&api_err).unwrap();

        assert!(json["title"].is_string());
        assert!(json["correlation_id"].is_string());
        assert!(json["git_hash"].is_string());
        assert!(json["docs_url"].is_string());
    }

    #[test]
    fn test_api_error_omits_none_fields() {
        let err = LibError::NetworkError { timeout: 5 };
        let lib_report = LibReport(Report::new(err));
        let api_err = lib_report.to_api_error();

        let json = serde_json::to_value(&api_err).unwrap();

        // NetworkError has no code or help
        assert!(!json.as_object().unwrap().contains_key("code"));
        assert!(!json.as_object().unwrap().contains_key("help"));
    }

    #[test]
    fn test_api_error_includes_optional_fields() {
        let err = LibError::ConfigParseError {
            path: "api.json".to_string(),
            src: NamedSource::new("api.json", "{}".to_string()),
            span: (0, 1).into(),
        };
        let lib_report = LibReport(Report::new(err));
        let api_err = lib_report.to_api_error();

        let json = serde_json::to_value(&api_err).unwrap();

        assert!(json["code"].is_string());
        assert!(json["help"].is_string());
    }

    #[test]
    fn test_history_serializes_as_flat_array() {
        let err = LibError::NetworkError { timeout: 1 };
        let report = Report::new(err)
            .attach("First context")
            .attach("Second context");
        let lib_report = LibReport(report);
        let api_err = lib_report.to_api_error();

        let json = serde_json::to_value(&api_err).unwrap();

        assert!(json["history"].is_array());
        let history = json["history"].as_array().unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].as_str().unwrap(), "First context");
        assert_eq!(history[1].as_str().unwrap(), "Second context");
    }
}

#[cfg(test)]
mod integration_tests {
    use errors_lib::{perform_task, LibReport, ReportExt};

    #[test]
    fn test_perform_task_returns_error() {
        let result = perform_task();
        assert!(result.is_err());
    }

    #[test]
    fn test_perform_task_error_has_correct_type() {
        let result = perform_task();
        let err = result.unwrap_err();

        assert!(err.0.current_context().to_string().contains("config.json"));
    }

    #[test]
    fn test_perform_task_error_has_attachment() {
        let result = perform_task();
        let err = result.unwrap_err();

        let api_err = err.to_api_error();
        assert!(api_err.history.len() > 0);
        assert!(api_err.history[0]
            .message
            .contains("cannot proceed without a valid config"));
    }

    #[test]
    fn test_end_to_end_error_pipeline() {
        // Simulate the full error handling pipeline
        let result = perform_task();
        assert!(result.is_err());

        let lib_report = result.unwrap_err();

        // 1. Test diagnostic info
        assert!(lib_report.code().is_some());
        assert!(lib_report.help().is_some());

        // 2. Test API error conversion
        let api_err = lib_report.to_api_error();
        assert_eq!(api_err.code, Some("config::invalid_format".to_string()));
        assert!(!api_err.correlation_id.is_empty());

        // 3. Test serialization
        let json = serde_json::to_string(&api_err).unwrap();
        assert!(json.contains("correlation_id"));
        assert!(json.contains("git_hash"));
    }
}

#[cfg(test)]
mod snapshot_tests {
    use errors_lib::{perform_task, ReportExt};
    use insta::assert_json_snapshot;

    #[test]
    fn test_api_error_snapshot() {
        let result = perform_task();
        let lib_report = result.unwrap_err();
        let mut api_err = lib_report.to_api_error();

        // Normalize dynamic fields for consistent snapshots
        api_err.correlation_id = "TEST-ID".to_string();
        api_err.git_hash = "abc123".to_string();

        assert_json_snapshot!(api_err, {
            ".docs_url" => "[docs_url]"
        });
    }

    #[test]
    fn test_network_error_snapshot() {
        use errors_lib::{types::LibError, LibReport, ReportExt};
        use rootcause::Report;

        let err = LibError::NetworkError { timeout: 30 };
        let lib_report = LibReport(Report::new(err).attach("Connection refused"));
        let mut api_err = lib_report.to_api_error();

        api_err.correlation_id = "NET-TEST".to_string();
        api_err.git_hash = "def456".to_string();

        assert_json_snapshot!(api_err, {
            ".docs_url" => "[docs_url]"
        });
    }

    #[test]
    fn test_complex_error_tree_snapshot() {
        use errors_lib::{types::LibError, LibReport, ReportExt};
        use miette::NamedSource;
        use rootcause::Report;

        let child_err = LibError::NetworkError { timeout: 5 };
        let parent_err = LibError::ConfigParseError {
            path: "database.json".to_string(),
            src: NamedSource::new("database.json", r#"{"host": "localhost"}"#.to_string()),
            span: (9, 9).into(),
        };

        let report = Report::new(parent_err)
            .with_child(Report::new(child_err))
            .attach("Failed to load database configuration")
            .attach("Using default settings");

        let lib_report = LibReport(report);
        let mut api_err = lib_report.to_api_error();

        api_err.correlation_id = "TREE-TEST".to_string();
        api_err.git_hash = "ghi789".to_string();

        assert_json_snapshot!(api_err, {
            ".docs_url" => "[docs_url]"
        });
    }
}

#[cfg(test)]
mod edge_cases {
    use errors_lib::{types::LibError, LibReport, ReportExt};
    use miette::NamedSource;
    use rootcause::Report;

    #[test]
    fn test_error_with_empty_source() {
        let err = LibError::ConfigParseError {
            path: "empty.json".to_string(),
            src: NamedSource::new("empty.json", "".to_string()),
            span: (0, 0).into(),
        };
        let lib_report = LibReport(Report::new(err));

        // Should not panic
        let api_err = lib_report.to_api_error();
        assert!(api_err.title.contains("empty.json"));
    }

    #[test]
    fn test_error_with_unicode_path() {
        let err = LibError::ConfigParseError {
            path: "配置.json".to_string(),
            src: NamedSource::new("配置.json", "{}".to_string()),
            span: (0, 1).into(),
        };
        let lib_report = LibReport(Report::new(err));

        let display = format!("{}", lib_report);
        assert!(display.contains("配置.json"));
    }

    #[test]
    fn test_error_with_very_long_attachment() {
        let long_msg = "x".repeat(10000);
        let err = LibError::NetworkError { timeout: 1 };
        let report = Report::new(err).attach(long_msg.clone());
        let lib_report = LibReport(report);

        let api_err = lib_report.to_api_error();
        assert_eq!(api_err.history[0].message, long_msg);
    }

    #[test]
    fn test_zero_timeout_network_error() {
        let err = LibError::NetworkError { timeout: 0 };
        let display = format!("{}", err);
        assert_eq!(display, "Network timeout after 0s");
    }

    #[test]
    fn test_error_with_special_characters_in_source() {
        let source = r#"{"key": "value\nwith\ttabs"}"#;
        let err = LibError::ConfigParseError {
            path: "special.json".to_string(),
            src: NamedSource::new("special.json", source.to_string()),
            span: (0, 5).into(),
        };
        let lib_report = LibReport(Report::new(err));

        // Should handle special characters gracefully
        let _api_err = lib_report.to_api_error();
    }
}

#[cfg(test)]
mod concurrent_error_handling {
    use errors_lib::{types::LibError, LibReport, ReportExt};
    use rootcause::Report;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_error_can_be_shared_across_threads() {
        let err = LibError::NetworkError { timeout: 10 };
        let report = Report::new(err).into_cloneable();
        let report_clone = report.clone();

        let handle = thread::spawn(move || {
            let lib_report = LibReport(report_clone);
            lib_report.to_api_error()
        });

        let lib_report = LibReport(report);
        let api_err1 = lib_report.to_api_error();
        let api_err2 = handle.join().unwrap();

        // Both should have the same title
        assert_eq!(api_err1.title, api_err2.title);
    }

    #[test]
    fn test_multiple_threads_creating_errors() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let err = LibError::NetworkError { timeout: i };
                    let lib_report = LibReport(Report::new(err));
                    lib_report.to_api_error()
                })
            })
            .collect();

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        assert_eq!(results.len(), 10);
        // Each should have a unique correlation ID
        let ids: Vec<_> = results.iter().map(|r| &r.correlation_id).collect();
        let unique_ids: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique_ids.len(), 10);
    }
}

#[cfg(test)]
mod diagnostic_compatibility {
    use errors_lib::{types::LibError, LibReport};
    use miette::{Diagnostic, NamedSource};
    use rootcause::Report;

    #[test]
    fn test_can_convert_to_miette_report() {
        let err = LibError::ConfigParseError {
            path: "test.json".to_string(),
            src: NamedSource::new("test.json", "{}".to_string()),
            span: (0, 1).into(),
        };
        let lib_report = LibReport(Report::new(err));

        // Should be convertible to miette::Report
        let _miette_report = miette::Report::new(lib_report);
    }

    #[test]
    fn test_severity_defaults_to_none() {
        let err = LibError::NetworkError { timeout: 5 };
        let lib_report = LibReport(Report::new(err));

        assert!(lib_report.severity().is_none());
    }

    #[test]
    fn test_related_returns_none() {
        let err = LibError::NetworkError { timeout: 5 };
        let lib_report = LibReport(Report::new(err));

        assert!(lib_report.related().is_none());
    }

    #[test]
    fn test_diagnostic_source_returns_none() {
        let err = LibError::NetworkError { timeout: 5 };
        let lib_report = LibReport(Report::new(err));

        assert!(lib_report.diagnostic_source().is_none());
    }
}
