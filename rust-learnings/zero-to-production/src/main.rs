use std::net::TcpListener;
use zerotoprod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to port 8000");
    run(listener)?.await
}
