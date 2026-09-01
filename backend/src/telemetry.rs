use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

// Builds a subscriber, but does not install it globally.
// Kept separate from `init_subscriber` so tests can build one
// pointed at a different sink without touching the global default.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> tracing_subscriber::fmt::MakeWriter<'a> + Send + Sync + 'static,
{
    // Reads RUST_LOG env var if set, otherwise falls back to `env_filter`.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Installs the given subscriber as the global default.
// Call this exactly once, at the start of `main`.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirects log records from crates using the older `log` crate
    // (many do) into `tracing`, so you get one unified log stream.
    tracing_log::LogTracer::init().expect("Failed to set logger");

    set_global_default(subscriber).expect("Failed to set subscriber");
}