use zero2prod::{config::get_config, startup::run};

use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "zero2prod".into(),
        // Output the formatted spans to stdout
        std::io::stdout,
    );

    // The method `with` is provided by SubscriberExt allowing us to add extensions for the `Subscriber`
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);

    set_global_default(subscriber).expect("Falied to set tracing subscriber");

    let config = get_config().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", config.application_port);

    run(addr.as_str()).await
}
