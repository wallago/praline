use futures_util::{StreamExt, future, pin_mut};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::prelude::*;

/// WebSocket connection.
pub(super) async fn conn(address: &str, port: &str) -> Result<()> {
    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();

    tokio::select! {
        result = read_stdin(stdin_tx) => {
            tracing::info!("Stdin reader stopped: {:?}", result);
        },
        result = relay(address, port,stdin_rx) =>  {
            tracing::info!("Stdin reader stopped: {:?}", result);
        }
    }
    Ok(())
}

// Connects to the bridge and relays messages until either direction ends.
///
/// Stdin lines are forwarded to the websocket; every incoming message is
/// printed to stdout, and text messages are also published on `sink`.
///
/// # Errors
///
/// Returns an error if the websocket connection cannot be established.
async fn relay(address: &str, port: u16, stdin_rx: UnboundedReceiver<Message>) -> Result<()> {
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

/// Forwards stdin to `tx` as binary websocket messages, in chunks of up to 1 KiB.
///
/// On EOF or a read error it never returns, so a missing stdin (e.g. `/dev/null`
/// under systemd) doesn't end the websocket session.
///
/// # Errors
///
/// Returns an error if `tx`'s receiver has been dropped.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) -> Result<()> {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf))?;
    }

    // Keep `tx` alive: dropping it would end `stdin_to_ws` and close the socket.
    std::future::pending::<()>().await;
    Ok(())
}
