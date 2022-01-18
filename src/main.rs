use zero2prod::run;

#[tokio::main]
async fn main() {
    run("127.0.0.1:8000").await
}
