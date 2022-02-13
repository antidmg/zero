use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Compose tracing subscriber layers.
/// `Send` and `Sync` are used to pass the subscriber to `init_subscriber` later.
/// The `Sink` parameter allows us to customize which log sinks to use.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // Higher-ranked trait bound
    // Sink implements the `MakeWriter` trait for all choices of the liftime parameter `'a`
    // `MakeWriter` is a type that can create `io::Write` instances
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let formatting_layer = BunyanFormattingLayer::new(name, sink);

    // The method `with` is provided by SubscriberExt allowing us to add extensions for the `Subscriber`
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

/// Register a subscriber as the global default to process span data - only call this once.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // This redirects all `log` events to our tracing subscriber to get processed
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Falied to set tracing subscriber");
}
