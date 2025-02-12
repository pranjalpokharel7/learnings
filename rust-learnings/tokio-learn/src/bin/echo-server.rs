use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:5000").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // copy data here

            // this fails to compile because multiple mutable references at the same time
            // io::copy(&mut socket, &mut socket).await

            echo(socket).await;
        });
    }
}

async fn echo_io_copy(mut socket: TcpStream) {
    // split socket into reader and writer handle
    let (mut rd, mut wr) = socket.split();

    if io::copy(&mut rd, &mut wr).await.is_err() {
        eprintln!("failed to copy");
    };
}

async fn echo(mut socket: TcpStream) {
    let mut buf = vec![0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => {
                if socket.write(&buf[..n]).await.is_err() {
                    return;
                }
            }
            Err(_) => return, // unexpected socket error - return
        }
    }
}