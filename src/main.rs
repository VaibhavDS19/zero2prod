use std::net::TcpListener;
use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0") // Just to get to next line
        .expect("failed to connect to random port");
    run(listener)?.await
}
