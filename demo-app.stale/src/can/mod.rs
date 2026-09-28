use std::collections::HashSet;

use socketcan::{CanFilter, CanFrame, EmbeddedFrame, Frame, SocketOptions, tokio::CanSocket};
use tokio::sync::broadcast::{Receiver, Sender};

use crate::prelude::*;

/// A CAN frame as an (id, data) pair.
pub type CanMessage = (u32, Vec<u8>);

/// A `SocketCAN` interface, shared by one reader and one writer.
#[derive(Debug)]
pub(super) struct Can {
    /// Socket bound to the interface; reads and writes both go through it.
    socket: CanSocket,
}

impl Can {
    /// Opens the CAN interface `ifname`.
    ///
    /// With `expected_frames`, the kernel only delivers those IDs. '
    /// With `None`, every frame on the bus is received.
    ///
    /// # Errors
    ///
    /// Returns an error if the interface can't be opened, if an entry of
    /// `expected_frames` isn't valid hex, or if the filters can't be set.
    pub(crate) fn new(ifname: &str, expected_frames: Option<&HashSet<&str>>) -> Result<Self> {
        let socket = CanSocket::open(ifname)?;
        if let Some(frames) = expected_frames {
            let mask = 0x7FF;
            let mut filters = Vec::new();
            for frame in frames {
                let id = u32::from_str_radix(frame, 16)?;
                filters.push(CanFilter::new(id, mask));
            }
            socket.set_filters(&filters)?;
        }

        Ok(Self { socket })
    }

    /// Forwards every frame read from the bus to `sink` as an `(id, data)`
    /// pair, until the socket stops yielding frames.
    ///
    /// # Errors
    ///
    /// Returns an error if `sink` has no subscribers left.
    pub(crate) async fn listening(&self, sink: Sender<CanMessage>) -> Result<()> {
        while let Ok(frame) = self.socket.read_frame().await {
            let message = (frame.raw_id(), frame.data().to_vec());
            sink.send(message)?;
        }
        Ok(())
    }

    /// Writes every `(id, data)` pair received on `source` to the bus, until
    /// the channel closes. A pair that isn't a valid CAN frame (ID out of
    /// range, too much data) is logged and skipped.
    ///
    /// # Errors
    ///
    /// Returns an error if writing a frame to the socket fails.
    pub(crate) async fn writing(&self, mut source: Receiver<CanMessage>) -> Result<()> {
        while let Ok((id, data)) = source.recv().await {
            if let Some(frame) = CanFrame::from_raw_id(id, &data) {
                self.socket.write_frame(frame).await?;
            } else {
                tracing::warn!("invalid CAN frame: id={id:#x}, {} bytes", data.len());
            }
        }
        Ok(())
    }
}
