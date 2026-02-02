# Quick Start: Running Tests

## Installation

Place `tests.rs` in your crates/errors-lib directory as either:
- `crates/errors-lib/tests/lib_tests.rs` (integration test)
- OR add to `crates/errors-lib/src/lib.rs` (inline with `#[cfg(test)]`)

## Quick Commands

```bash
# Run all tests
cargo test

# Run specific test module  
cargo test error_construction
cargo test serialization
cargo test --test lib_tests  # if using tests/ directory

# Run with output
cargo test -- --nocapture

# Run snapshot tests
cargo insta test
cargo insta review
cargo insta accept

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html

# Run with nextest (faster)
cargo nextest run
```

## Expected Output

```
running 50 tests
test error_construction::test_config_parse_error_display ... ok
test error_construction::test_network_error_display ... ok
test error_construction::test_config_error_has_diagnostic_code ... ok
test error_construction::test_config_error_has_help_text ... ok
test error_construction::test_error_source_code_available ... ok
test error_construction::test_error_has_labels ... ok
test report_wrapper::test_lib_report_wraps_rootcause_report ... ok
test report_wrapper::test_lib_report_implements_diagnostic ... ok
test report_wrapper::test_lib_report_url_generation ... ok
test report_wrapper::test_lib_report_implements_error_trait ... ok
test report_extension_trait::test_to_api_error_basic ... ok
test report_extension_trait::test_to_api_error_with_metadata ... ok
test report_extension_trait::test_api_error_contains_git_hash ... ok
test report_extension_trait::test_api_error_contains_docs_url ... ok
test report_extension_trait::test_api_error_with_attachments ... ok
test error_tree_navigation::test_tree_iteration_single_error ... ok
test error_tree_navigation::test_tree_iteration_with_children ... ok
test error_tree_navigation::test_handle_error_logic_with_io_error ... ok
test error_tree_navigation::test_downcast_to_specific_error_type ... ok
test serialization::test_api_error_serializes_to_json ... ok
test serialization::test_api_error_omits_none_fields ... ok
test serialization::test_api_error_includes_optional_fields ... ok
test serialization::test_history_serializes_as_flat_array ... ok
test integration_tests::test_perform_task_returns_error ... ok
test integration_tests::test_perform_task_error_has_correct_type ... ok
test integration_tests::test_perform_task_error_has_attachment ... ok
test integration_tests::test_end_to_end_error_pipeline ... ok
test snapshot_tests::test_api_error_snapshot ... ok
test snapshot_tests::test_network_error_snapshot ... ok
test snapshot_tests::test_complex_error_tree_snapshot ... ok
test edge_cases::test_error_with_empty_source ... ok
test edge_cases::test_error_with_unicode_path ... ok
test edge_cases::test_error_with_very_long_attachment ... ok
test edge_cases::test_zero_timeout_network_error ... ok
test edge_cases::test_error_with_special_characters_in_source ... ok
test concurrent_error_handling::test_error_can_be_shared_across_threads ... ok
test concurrent_error_handling::test_multiple_threads_creating_errors ... ok
test diagnostic_compatibility::test_can_convert_to_miette_report ... ok
test diagnostic_compatibility::test_severity_defaults_to_none ... ok
test diagnostic_compatibility::test_related_returns_none ... ok
test diagnostic_compatibility::test_diagnostic_source_returns_none ... ok

test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Test Categories

| Category | Count | Purpose |
|----------|-------|---------|
| 🏗️ Construction | 6 | Error creation & display |
| 📦 Wrapper | 4 | LibReport wrapper |
| 🔄 Extension | 5 | ReportExt trait |
| 🌳 Tree Navigation | 4 | Error tree ops |
| 📊 Serialization | 4 | JSON conversion |
| 🔗 Integration | 4 | End-to-end |
| 📸 Snapshots | 3 | Regression testing |
| ⚠️ Edge Cases | 5 | Boundary conditions |
| 🧵 Concurrency | 2 | Thread safety |
| 🔍 Diagnostics | 4 | Miette integration |

## Snapshot Testing

The test suite includes snapshot tests that capture the exact JSON output. When you first run:

```bash
cargo insta test
```

Review snapshots:
```bash
cargo insta review
```

This will show you each snapshot and let you accept/reject:
```
────────────────────────────────────────────────────────────
Snapshot: test_api_error_snapshot
────────────────────────────────────────────────────────────
{
  "git_hash": "abc123",
  "docs_url": "[docs_url]",
  "correlation_id": "TEST-ID",
  "title": "Failed to parse config at config.json",
  "code": "config::invalid_format",
  "help": "Ensure the configuration file is valid JSON.",
  "history": [
    "The application cannot proceed without a valid config."
  ]
}

(a)ccept, (r)eject, (s)kip, (b)ackspace, (q)uit
```

## Troubleshooting

### "unused import" warnings
The test modules use feature-gated imports. Run with:
```bash
cargo test --all-features
```

### Snapshot mismatches
This is normal on first run. Review and accept:
```bash
cargo insta review
cargo insta accept
```

### Git hash shows "unknown"
The build script uses git. Either:
1. Ensure you're in a git repository
2. Or the test will use "unknown" - this is fine

### Parallel test conflicts
If tests conflict (rare), run sequentially:
```bash
cargo test -- --test-threads=1
```

## CI Integration

Add to `.github/workflows/test.yml`:
```yaml
name: Test Suite
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
```

## Next Steps

1. Place `tests.rs` in your project
2. Run `cargo test`
3. Review snapshot tests with `cargo insta review`
4. Add to CI pipeline
5. Keep tests updated as you add features

## Support

For issues or questions:
- Check TEST_DOCUMENTATION.md for detailed info
- Review individual test comments
- Ensure all dependencies are in Cargo.toml
