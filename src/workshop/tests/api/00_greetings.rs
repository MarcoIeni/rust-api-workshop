//! Welcome to the workshop! 👋
//! You will learn how to write Rust HTTP API. 🦀
//!
//! The material is structured as a series of exercises.

/// This is your first exercise! 🤓
/// In this case, the test contains a `todo!()`.
/// If a `todo!()` is present in a test, you are expected to replace it with working code.
/// Be aware, sometimes a one-liner won't be enough!
///
/// Now:
/// - Comment out this module in the `src/workshop/tests/api/main.rs` file.
/// - Open your terminal.
/// - cd into the `src/workshop` directory.
/// - Run `cargo test`: you should see some errors.
/// - Make the test pass.
/// - Run `cargo test` to check if the test works.
/// - Comment out the next module in the `src/workshop/tests/api/main.rs` file to progress.
#[test]
fn first_exercise() {
    let i_am_ready_to_start = todo!();

    assert!(i_am_ready_to_start);
}
