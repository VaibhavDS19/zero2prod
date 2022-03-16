use std::net::TcpListener;
use zero2prod::run;
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    println!(
        "Running port number {}",
        listener.local_addr().unwrap().port()
    );
    run(listener)?.await
}
