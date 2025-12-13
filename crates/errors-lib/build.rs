/*
 * Build script to inject environment metadata and Git state.
 * Handles PKG_VERSION for documentation and GIT_HASH for version tracking.
 */

use std::process::Command;

fn main() {
    // 1. Documentation Metadata
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    let version = env!("CARGO_PKG_VERSION");
    println!("cargo:rustc-env=ERROR_DOCS_URL=https://docs.rs/errors-lib/{version}");

    // 2. Git Metadata
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output();

    let git_hash = match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_string(),
        _ => "unknown".to_string(),
    };

    println!("cargo:rustc-env=GIT_HASH={git_hash}");
    println!("cargo:rerun-if-changed=.git/HEAD");
}
