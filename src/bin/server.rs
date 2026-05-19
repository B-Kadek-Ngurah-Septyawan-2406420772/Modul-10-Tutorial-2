use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{Sender, channel};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();
    ws_stream
        .send(Message::text(
            "Awan's Computer - From server: Welcome to chat! Type a message",
        ))
        .await?;

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                let Some(message) = incoming else {
                    break;
                };

                let message = message?;
                if let Some(text) = message.as_text() {
                    let sender_message = format!("{addr}: {text}");
                    println!("From client {addr} \"{text}\"");
                    bcast_tx.send(format!("Awan's Computer - From server: {sender_message}"))?;
                }
            }
            broadcast = bcast_rx.recv() => {
                let message = broadcast?;
                ws_stream.send(Message::text(message)).await?;
            }
        }
    }

    println!("{addr} disconnected");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from Awan's Computer {addr}");
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}
