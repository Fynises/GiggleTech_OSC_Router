use std::{collections::HashMap, sync::Arc, time::Duration};
use anyhow::Result;
use async_osc::OscSocket;
use tokio::task::JoinHandle;
use crate::config::Config;

const TX_OSC_MOTOR_ADDRESS: &str = "/avatar/parameters/motor"; 
const TX_OSC_GIGGLESPARK: &str = "/motor"; 

#[derive(Debug)]
pub struct DeviceSocket {
    /// tx socket is wrapped in an Arc to allow for timeout functionality to function in a
    /// clean fashion, not sure if doing this is safe or not.
    tx_socket: Arc<OscSocket>,
    timeout_duration: u64,
    timeout_join_handle: Option<JoinHandle<()>>,
    address: String,
}

impl DeviceSocket {
    /// creates this socket object
    /// panics on error connecting
    pub async fn new(
        device_ip: String,
        port: String,
        timeout_duration: u64,
        target_address: String
    ) -> Self {
        let address = format!("{}:{}", device_ip, port);
        let socket = OscSocket::bind("0.0.0.0:0").await.unwrap();
        socket.connect(address).await.unwrap();
        log::info!("building device socket with address {}", target_address);
        Self {
            tx_socket: Arc::new(socket),
            timeout_duration,
            timeout_join_handle: None,
            address: target_address
        }
    }

    /// sends a packet to this device
    pub async fn send(&mut self, payload: i32) -> Result<()> {
        self.reset_timeout();
        self.tx_socket.send((&self.address, (payload,))).await?;
        Ok(())
    }

    fn reset_timeout(&mut self) {
        self.timeout_join_handle.take().map(|j| j.abort());

        let timeout_duration = self.timeout_duration;
        let socket = self.tx_socket.clone();
        let address = self.address.clone();

        let join_handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(timeout_duration)).await;
            match socket.send((address, (0i32,))).await {
                Ok(_) => (),
                Err(e) => log::error!("error occurred sending timeout signal {}", e),
            };
        });

        self.timeout_join_handle = Some(join_handle);
    }

    pub async fn from_config(config: Arc<Config>) -> HashMap<String, Vec<Self>> {
        let mut socket_map: HashMap<String, Vec<Self>> = HashMap::new();
        for address in config.headpat_device_uris.iter() {
            let index = config.headpat_device_uris.iter().position(|a| *a == address.clone());
            let mut sockets: Vec<Self> = Vec::new();
            sockets.push(Self::new(address.clone(), String::from("8888"), config.timeout_setting, TX_OSC_MOTOR_ADDRESS.to_string()).await);
            sockets.push(Self::new(address.clone(), String::from("8888"), config.timeout_setting, TX_OSC_GIGGLESPARK.to_string()).await);
            socket_map.insert(config.proximity_parameters_multi[index.unwrap()].clone(), sockets);
        }
        socket_map
    }
}
