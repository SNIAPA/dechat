use anyhow::Result;
use tokio::{net::{TcpListener, TcpSocket, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};

use crate::PORT;

pub async fn listen() -> Result<()>{
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await?;
    }
}

pub async fn process_socket(mut stream: TcpStream) -> Result<()>{
    dbg!(1);
    let mut buf = String::new();
    stream.read_to_string(&mut buf).await?;
    dbg!(buf);
    dbg!(4);
    stream.write(b"test ans\n\r").await?;
    stream.flush().await?;

    Ok(())
}
