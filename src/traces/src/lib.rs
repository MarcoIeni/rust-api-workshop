use once_cell::sync::Lazy;
use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn init_test_traces() {
    // Only initialize once logs once
    Lazy::force(&TRACING);
}

// Initialize the `tracing` stack once, using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("ENABLE_LOGS").is_ok() {
        let default_filter_level = "info";
        let subscriber = get_subscriber(default_filter_level);
        init_subscriber(subscriber);
    }
});

fn get_subscriber(env_filter: &str) -> impl Subscriber + Sync + Send {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new("yoda-taller".to_string(), std::io::stdout);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
