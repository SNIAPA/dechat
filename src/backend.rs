use anyhow::Result;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpSocket, TcpStream},
};

use crate::PORT;

pub async fn listen() -> Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT)).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await?;
    }
}

pub async fn process_socket(mut stream: TcpStream) -> Result<()> {
    dbg!(21);
    stream.write(b"test").await?;
    dbg!(22);
    stream.flush().await?;
    dbg!(23);
    let mut buf = String::new();
    dbg!(24);
    stream.read_to_string(&mut buf).await?;
    dbg!(25);
    stream.flush().await?;
    dbg!(26);


    Ok(())
}
