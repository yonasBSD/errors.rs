#[cfg(not(target_arch = "wasm32"))]
use human_panic::{metadata, setup_panic};

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(target_arch = "wasm32"))]
    setup_panic!(metadata!()
        .authors("Acme Inc. <support@example.com")
        .homepage("www.example.com")
        .support("- Open a support request by email to support@example.com"));

    assert_eq(1, 2);
}
