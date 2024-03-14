use std::collections::HashMap;
use crate::device_actor::device_socket_handle::DeviceSocketHandle;
use anyhow::Result;
use tokio::sync::mpsc::UnboundedReceiver;

/// main actor provides a safe interface to the giggletech device
pub struct MainActor {
    rx: UnboundedReceiver<(String, i32)>,
    device_sockets: HashMap<String, DeviceSocketHandle>,
}

impl MainActor {
    pub fn new(rx: UnboundedReceiver<(String, i32)>, device_sockets: HashMap<String, DeviceSocketHandle>) -> Self {
        Self { rx, device_sockets }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.rx.recv().await {
            match self.handle_send_packet(msg).await {
                Ok(_) => (),
                Err(e) => log::error!("error occurred in main actor: {}", e),
            };
        }
    }

    /// handles sending of the packet
    async fn handle_send_packet(&mut self, payload: (String, i32)) -> Result<()> {
        let (device_address, packet) = payload;
        match self.device_sockets.get_mut(&device_address) {
            Some(device_socket) => {
                device_socket.send(packet)?
            },
            None => log::warn!("cannot find device id: {}", device_address),
        }
        Ok(())
    }
}
