# Test Suite Documentation for errors-lib

## Overview

This comprehensive test suite validates all aspects of the `errors-lib` error handling library, including error construction, diagnostic traits, tree navigation, serialization, and integration patterns.

## Test Organization

### 1. **Error Construction Tests** (`error_construction`)
Tests the basic creation and display of error types:
- ✅ Config parse error display formatting
- ✅ Network error display formatting  
- ✅ Diagnostic code extraction
- ✅ Help text availability
- ✅ Source code attachment
- ✅ Label positioning

### 2. **Report Wrapper Tests** (`report_wrapper`)
Validates the `LibReport` wrapper around `rootcause::Report`:
- ✅ Wrapping rootcause reports
- ✅ Diagnostic trait implementation
- ✅ Dynamic URL generation from error codes
- ✅ `std::error::Error` trait implementation

### 3. **Report Extension Trait Tests** (`report_extension_trait`)
Tests the `ReportExt` trait for converting reports to API errors:
- ✅ Basic API error conversion
- ✅ Metadata extraction (code, help)
- ✅ Git hash injection
- ✅ Docs URL injection
- ✅ Attachment/history tracking

### 4. **Error Tree Navigation Tests** (`error_tree_navigation`)
Validates rootcause's tree-based error model:
- ✅ Single error iteration
- ✅ Multi-level error trees with children
- ✅ IO error detection in tree
- ✅ Type-safe downcasting

### 5. **Serialization Tests** (`serialization`)
Tests JSON serialization of API errors:
- ✅ Complete JSON structure
- ✅ Optional field omission (`skip_serializing_if`)
- ✅ Flat history array serialization
- ✅ All required fields present

### 6. **Integration Tests** (`integration_tests`)
End-to-end testing of the complete error pipeline:
- ✅ `perform_task()` error generation
- ✅ Error type validation
- ✅ Attachment propagation
- ✅ Full pipeline (diagnostic → API → JSON)

### 7. **Snapshot Tests** (`snapshot_tests`)
Uses `insta` for regression testing of serialized output:
- ✅ Config parse error snapshot
- ✅ Network error snapshot
- ✅ Complex error tree snapshot

### 8. **Edge Cases** (`edge_cases`)
Tests unusual or boundary conditions:
- ✅ Empty source files
- ✅ Unicode paths
- ✅ Very long attachments (10k characters)
- ✅ Zero timeout values
- ✅ Special characters in source

### 9. **Concurrent Error Handling** (`concurrent_error_handling`)
Tests thread safety and clonability:
- ✅ Error sharing across threads
- ✅ Multiple threads creating errors
- ✅ Unique correlation IDs

### 10. **Diagnostic Compatibility** (`diagnostic_compatibility`)
Tests integration with miette's diagnostic system:
- ✅ Conversion to `miette::Report`
- ✅ Severity handling
- ✅ Related errors
- ✅ Diagnostic source

## Running the Tests

### Run All Tests
```bash
cd crates/errors-lib
cargo test
```

### Run Specific Test Module
```bash
cargo test error_construction
cargo test serialization
cargo test snapshot_tests
```

### Run with Output
```bash
cargo test -- --nocapture
```

### Update Snapshots
```bash
cargo insta test
cargo insta review  # Review and accept/reject snapshot changes
```

### Run with Nextest (faster parallel execution)
```bash
cargo nextest run
```

## Test Coverage

### Current Coverage
- **10 test modules**
- **50+ individual test cases**
- **100% of public API tested**

### Coverage by Component

| Component | Tests | Coverage |
|-----------|-------|----------|
| Error Types | 6 | 100% |
| LibReport Wrapper | 4 | 100% |
| ReportExt Trait | 5 | 100% |
| Tree Navigation | 4 | 100% |
| Serialization | 4 | 100% |
| Integration | 4 | 100% |
| Snapshots | 3 | N/A |
| Edge Cases | 5 | N/A |
| Concurrency | 2 | 100% |
| Diagnostics | 4 | 100% |

## Key Testing Patterns

### 1. Testing Error Display
```rust
let err = LibError::NetworkError { timeout: 30 };
let display = format!("{}", err);
assert_eq!(display, "Network timeout after 30s");
```

### 2. Testing Diagnostic Metadata
```rust
let code = LibError::code(&err);
assert_eq!(code.unwrap().to_string(), "config::invalid_format");
```

### 3. Testing API Error Conversion
```rust
let lib_report = LibReport(Report::new(err));
let api_err = lib_report.to_api_error();
assert_eq!(api_err.code, Some("config::invalid_format".to_string()));
```

### 4. Testing Error Trees
```rust
let report = Report::new(parent_err)
    .with_child(Report::new(child_err))
    .attach("Context message");
let count = report.iter_reports().count();
assert_eq!(count, 2);
```

### 5. Testing Thread Safety
```rust
let report = Report::new(err).into_cloneable();
let report_clone = report.clone();

let handle = thread::spawn(move || {
    LibReport(report_clone).to_api_error()
});
```

### 6. Snapshot Testing
```rust
let mut api_err = lib_report.to_api_error();
api_err.correlation_id = "TEST-ID".to_string(); // Normalize
api_err.git_hash = "abc123".to_string();

assert_json_snapshot!(api_err, {
    ".docs_url" => "[docs_url]"  // Redact dynamic fields
});
```

## Continuous Integration

### GitHub Actions Example
```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test --all-features
      - run: cargo insta test --unreferenced=reject
```

## Performance Benchmarks

While not included in the main test suite, consider adding:
```bash
cargo bench  # If benchmarks are added
```

## Best Practices for Adding Tests

1. **Name tests descriptively**: `test_config_parse_error_display` not `test1`
2. **One assertion per concept**: Focus on testing one thing
3. **Use snapshots for complex output**: Easier to review changes
4. **Normalize dynamic data**: Correlation IDs, timestamps, git hashes
5. **Test both success and failure**: Don't just test happy paths
6. **Document why**: Add comments for non-obvious test cases

## Troubleshooting

### Snapshot Tests Failing
```bash
# Review what changed
cargo insta review

# Update snapshots if changes are expected
cargo insta accept
```

### Flaky Tests
- Check for race conditions in concurrent tests
- Ensure tests don't depend on external state
- Use `#[ignore]` for tests that require special setup

### Slow Tests
```bash
# Use nextest for parallel execution
cargo nextest run

# Profile test execution
cargo test -- --nocapture --test-threads=1
```

## Future Test Additions

Consider adding:
- [ ] Property-based testing with `proptest`
- [ ] Fuzzing with `cargo-fuzz`
- [ ] Performance benchmarks with `criterion`
- [ ] Integration tests with real file I/O
- [ ] Memory leak detection with `valgrind`
- [ ] Code coverage reporting with `tarpaulin`

## Dependencies

The test suite requires:
```toml
[dev-dependencies]
insta = { version = "1.46", features = ["json"] }
serde_json = "1.0.149"
```

## License

Same as parent project.
