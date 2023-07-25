use tracing::{info, warn};

/// During your journey, you will need logs to troubleshoot issues.
/// Let's see how to use the [tracing](https://docs.rs/tracing) crate,
/// to see the logs of an application.
#[test]
fn i_can_see_logs() {
    init_test_logs();

    info!("Hello, world!");
    my_fn("Bob");

    // Run the test with the `TEST_LOG` environment variable set to see the logs:
    // ```sh
    // $ TEST_LOG=1 cargo test i_can_see_logs
    // ```
    let i_can_see_logs: bool = todo!();
    assert!(i_can_see_logs);
}

fn init_test_logs() {
    // We only want to print logs when the `TEST_LOG` environment variable is set.
    // Otherwise, it would be hard to read the test output.
    if std::env::var("TEST_LOG").is_ok() {
        // Construct a subscriber that prints formatted traces to stdout.
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // Use that subscriber to process traces emitted after this point.
        tracing::subscriber::set_global_default(subscriber).unwrap();
    }
}

#[tracing::instrument]
fn my_fn(_name: &str) {
    warn!("Hello from my_fn!");
}

// 💡 When you want to inspect the logs in your tests, copy paste the `init_test_logs()` function.
