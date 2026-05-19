use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) = ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
        .connect()
        .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            line = stdin.next_line() => {
                match line {
                    Ok(Some(line)) => ws_stream.send(Message::text(line)).await?,
                    Ok(None) => break,
                    Err(err) => {
                        eprintln!("failed to read from stdin: {err}");
                        break;
                    }
                }
            }
            message = ws_stream.next() => {
                let Some(message) = message else {
                    break;
                };

                let message = message?;
                if let Some(text) = message.as_text() {
                    println!("{text}");
                }
            }
        }
    }

    Ok(())
}
