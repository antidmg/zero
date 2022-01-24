use zero2prod::{config::get_config, startup::run};

#[tokio::main]
async fn main() {
    let config = get_config().expect("Failed to read configuration.");
    let addr = format!("127.0.0.1:{}", config.application_port);

    run(addr.as_str()).await
}
