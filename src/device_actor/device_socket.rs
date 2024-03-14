use std::time::Duration;
use anyhow::Result;
use async_osc::OscSocket;
use tokio::{sync::mpsc::UnboundedReceiver, time::Interval};

#[derive(Debug)]
pub struct DeviceSocket {
    rx: UnboundedReceiver<i32>,
    timeout_interval: Interval,
    tx_socket: OscSocket,
    address: String,
}

impl DeviceSocket {
    /// creates this socket object
    /// panics on error connecting
    pub fn new(
        rx: UnboundedReceiver<i32>,
        socket: OscSocket,
        target_address: String,
        timeout_duration: u64,
    ) -> Self {
        let interval = tokio::time::interval(Duration::from_secs(timeout_duration));
        Self {
            rx,
            timeout_interval: interval,
            tx_socket: socket,
            address: target_address
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        self.timeout_interval.tick().await;
        loop {
            tokio::select! {
                res = self.rx.recv() => {
                    if let Some(packet) = res { 
                        self.send(packet).await?;
                        self.timeout_interval.reset();
                    }
                },
                _res = self.timeout_interval.tick() => {
                    self.send(0).await?;
                }
            }
        }
    }

    /// sends a packet to this device
    pub async fn send(&self, payload: i32) -> Result<()> {
        self.tx_socket.send((&self.address, (payload,))).await?;
        Ok(())
    }
}
