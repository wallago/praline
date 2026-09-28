use std::collections::{HashMap, HashSet};

use socketcan::{CanFilter, CanFrame, EmbeddedFrame, Frame, SocketOptions, tokio::CanSocket};
use tokio::sync::{
    broadcast::{self, Sender},
    mpsc,
};

use crate::{config::AppConfig, prelude::*};

/// A CAN frame as an (id, data) pair.
pub type CanMessage = (u32, Vec<u8>);

/// CAN gateway to send or receive frames.
#[derive(Clone)]
pub struct BusHandle {
    /// CAN -> app: call .subscribe() to receive frames from the bus.
    pub frames: broadcast::Sender<CanMessage>,
    /// app -> CAN: send (id, data) here to emit a frame on the bus.
    pub send: mpsc::Sender<CanMessage>,
}

/// A CAN bus interface that reads frames from a SocketCAN socket and
/// fans them out to subscribers over a broadcast channel.
#[derive(Debug)]
struct Can {
    /// Context name this bus is bridged to, e.g. "gateway".
    pub name: String,
    ifname: String,
    /// CAN frame transceiver (broadcast mode).
    pub tx: Sender<CanMessage>,
}

impl Can {
    /// Opens the CAN interface `ifname` and prepares the broadcast channel.
    pub fn new(
        name: String,
        ifname: String,
        expected_frames: Option<HashSet<String>>,
    ) -> Result<Self> {
        let socket = CanSocket::open(&ifname)?;
        if let Some(ref frames) = expected_frames {
            let mask = 0x7FF;
            let mut filters = Vec::new();
            for frame in frames {
                let id = u32::from_str_radix(frame, 16)?;
                filters.push(CanFilter::new(id, mask));
            }
            socket.set_filters(&filters)?;
        }

        let (tx, _) = broadcast::channel::<CanMessage>(100);
        let can = Self { name, ifname, tx };

        can.spawn_listener(socket);

        Ok(can)
    }

    /// Spawns a task that reads frames from the socket and broadcasts them to subscribers.
    fn spawn_listener(&self, socket: CanSocket) {
        let tx = self.tx.clone();
        tokio::spawn(async move {
            while let Ok(frame) = socket.read_frame().await {
                let id = frame.raw_id();
                let data = frame.data().to_vec();
                if let Err(err) = tx.send((id, data)) {
                    tracing::trace!("Broadcasting message failed: {err}");
                }
            }
        });
    }

    /// Spawns a task that writes frames from the channel and send them to the bus.
    pub fn spawn_writer(&self) -> Result<mpsc::Sender<CanMessage>> {
        let socket = CanSocket::open(&self.ifname)?;
        let (tx, mut rx) = mpsc::channel::<CanMessage>(100);
        tokio::spawn(async move {
            while let Some((id, data)) = rx.recv().await {
                match CanFrame::from_raw_id(id, &data) {
                    Some(frame) => {
                        if let Err(err) = socket.write_frame(frame).await {
                            tracing::error!("CAN write failed: {err}");
                        }
                    }
                    None => tracing::warn!("invalid CAN frame: id={id:#x}, {} bytes", data.len()),
                }
            }
        });
        Ok(tx)
    }
}

/// Start CAN buses.
pub fn start(config: &AppConfig) -> Result<HashMap<String, BusHandle>> {
    let mut buses = HashMap::new();
    for cfg in config.can.clone() {
        let frames_id = cfg.frames_id;
        let can = Can::new(
            cfg.name,
            cfg.bus,
            (!frames_id.is_empty()).then_some(frames_id),
        )?;
        buses.insert(
            can.name.clone(),
            BusHandle {
                frames: can.tx.clone(),
                send: can.spawn_writer()?,
            },
        );
    }
    Ok(buses)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serial_test::file_serial;
    use socketcan::{CanFrame, StandardId};

    use super::*;

    #[rstest]
    #[tokio::test]
    #[file_serial(vcan)]
    #[ignore = "requires a vcan0 interface"]
    async fn test_listen_broadcasts_received_frames() {
        let can = Can::new("test".to_string(), "vcan0".to_string(), None).expect("open vcan0");
        let mut rx = can.tx.subscribe();

        let tx = CanSocket::open("vcan0").expect("open sender socket");
        let frame = CanFrame::new(
            StandardId::new(0x333).expect("create CAN frame ID"),
            &[0xDE, 0xAD],
        )
        .expect("create CAN frame");
        tx.write_frame(frame).await.expect("write frame");

        let (id, data) = rx.recv().await.expect("frame not broadcast");
        assert_eq!(id, 0x333);
        assert_eq!(data, vec![0xDE, 0xAD]);
    }

    #[rstest]
    #[tokio::test]
    #[file_serial(vcan)]
    #[ignore = "requires a vcan0 interface"]
    async fn test_listen_only_receives_expected_frames() {
        let expected = HashSet::from(["333".to_string()]);
        let can =
            Can::new("test".to_string(), "vcan0".to_string(), Some(expected)).expect("open vcan0");
        let mut rx = can.tx.subscribe();

        let tx = CanSocket::open("vcan0").expect("open sender socket");
        let ignored = CanFrame::new(
            StandardId::new(0x111).expect("create CAN frame ID"),
            &[0x01],
        )
        .expect("create CAN frame");
        let wanted = CanFrame::new(
            StandardId::new(0x333).expect("create CAN frame ID"),
            &[0x02],
        )
        .expect("create CAN frame");
        tx.write_frame(ignored).await.expect("write frame");
        tx.write_frame(wanted).await.expect("write frame");

        // The 0x111 frame is dropped by the kernel filter, so the first
        // frame out of the channel must be 0x333.
        let (id, data) = rx.recv().await.expect("frame not broadcast");
        assert_eq!(id, 0x333);
        assert_eq!(data, vec![0x02]);
    }

    #[rstest]
    #[tokio::test]
    #[file_serial(vcan)]
    #[ignore = "requires a vcan0 interface"]
    async fn test_start() {
        let config = AppConfig {
            can: vec![crate::config::can::CanConfig {
                name: "gateway".to_string(),
                bus: "vcan0".to_string(),
                frames_id: HashSet::new(),
            }],
            ..Default::default()
        };

        let buses = start(&config).expect("start CAN buses");
        assert_eq!(buses.len(), 1);
        let handle = buses
            .get("gateway")
            .expect("bus registered under its config name");

        let mut rx = handle.frames.subscribe();
        handle
            .send
            .send((0x333, vec![0xDE, 0xAD]))
            .await
            .expect("queue frame");

        let (id, data) = tokio::time::timeout(std::time::Duration::from_secs(1), rx.recv())
            .await
            .expect("timed out waiting for frame")
            .expect("frame not broadcast");
        assert_eq!(id, 0x333);
        assert_eq!(data, vec![0xDE, 0xAD]);
    }

    #[test]
    fn test_start_fails_on_unknown_interface() {
        let config = toml::from_str::<AppConfig>(
            r#"
        [[can]]
        bus = "nonexistent0"
        name = "test"
        "#,
        )
        .expect("build config");
        assert!(start(&config).is_err());
    }
}
