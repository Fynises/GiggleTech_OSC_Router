use std::{collections::HashMap, sync::Arc};
use anyhow::Result;
use async_osc::OscSocket;
use tokio::sync::mpsc::{self, UnboundedSender};
use crate::config::Config;
use super::device_socket::DeviceSocket;

pub struct DeviceSocketHandle {
    tx: UnboundedSender<i32>,
}

impl DeviceSocketHandle {
    pub async fn new(device_ip: String, target_address: String, timeout: u64) -> Result<Self> {
        // setup osc socket;
        let address = format!("{}:8888", device_ip);
        let socket = OscSocket::bind("0.0.0.0:0").await?;
        socket.connect(address).await?;

        let (tx, rx) = mpsc::unbounded_channel::<i32>();
        let mut actor = DeviceSocket::new(rx, socket, target_address, timeout);
        let _ = tokio::spawn(async move { 
            if let Err(e) = actor.run().await {
                log::error!("error occurred in device actor {}", e)
            }
        });
        Ok(Self { tx })        
    }

    pub fn send(&self, packet: i32) -> Result<()> {
        self.tx.send(packet)?;
        Ok(())
    }

    pub async fn from_config(config: Arc<Config>) -> HashMap<String, Self> {
        let mut socket_map: HashMap<String, Self> = HashMap::new();
        
        for address in config.headpat_device_uris.iter() {
            let handle = Self::new(
                address.uri.clone(),
                address.device_type.as_address(),
                config.timeout_setting
            ).await.unwrap();
            socket_map.insert(address.uri.clone(), handle);
        }
        socket_map
    }
}
