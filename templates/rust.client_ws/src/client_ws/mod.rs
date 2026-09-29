use futures_util::{StreamExt, future, pin_mut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::prelude::*;

/// WebSocket connection.
pub(super) async fn conn(address: &str, port: &str) -> Result<()> {
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (ws_stream, _) = connect_async(&format!("ws://{address}:{port}/ws")).await?;
    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = read.for_each(|message| async move {
        match message {
            Ok(msg) => {
                let mut stdout = tokio::io::stdout();
                let mut line = msg.into_data().to_vec();
                if line.last() != Some(&b'\n') {
                    line.push(b'\n');
                }
                if let Err(e) = stdout.write_all(&line).await {
                    eprintln!("stdout write failed: {e}");
                }
                let _ = stdout.flush().await;
            }
            Err(e) => eprintln!("ws read failed: {e}"),
        }
    });

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
    Ok(())
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}
