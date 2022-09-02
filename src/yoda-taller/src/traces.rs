use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber(env_filter: &str) -> anyhow::Result<impl Subscriber + Sync + Send> {
    let service_name = "yoda-taller";
    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_service_name(service_name)
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(service_name.to_string(), std::io::stdout);
    let registry = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(telemetry)
        .with(formatting_layer);
    Ok(registry)
}

/// Register a subscriber as global default to process span data.
///
/// It should only be called once!
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}

pub fn init_prod_traces() -> anyhow::Result<()> {
    let subscriber = get_subscriber("info")?;
    init_subscriber(subscriber);
    Ok(())
}
