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
        tokio::spawn(async move {
            process_socket(socket).await.unwrap();
        });
    }
}

pub async fn process_socket(mut stream: TcpStream) -> Result<()> {

    Ok(())
}
