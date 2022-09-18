use {
    once_cell::sync::Lazy,
    yoda_taller::traces::{get_subscriber, init_subscriber},
};

pub fn init_test_traces() {
    // Initialize logs only once
    Lazy::force(&TRACING);
}

// Initialize the `tracing` stack once, using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let default_filter_level = "info";
        let subscriber = get_subscriber(default_filter_level).unwrap();
        init_subscriber(subscriber);
    }
});
