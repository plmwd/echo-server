use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("Accepted connection from {:?}", socket.peer_addr().unwrap());
            let (mut rd, mut wr) = socket.split();
            let mut buf = [0; 1024];
            rd.peek(&mut buf).await.unwrap();
            println!("{}", String::from_utf8_lossy(&buf));
            if io::copy(&mut rd, &mut wr).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}
