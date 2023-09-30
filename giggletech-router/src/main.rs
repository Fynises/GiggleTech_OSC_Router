// GiggleTech.io
// OSC Router
// by Sideways
// Based off OSC Async https://github.com/Frando/async-osc

use anyhow::Result;
use async_std::{stream::StreamExt, sync::Arc,};
use std::sync::atomic::AtomicBool;

use crate::osc_timeout::osc_timeout;
mod data_processing;
mod config;
mod giggletech_osc;
mod terminator;
mod osc_timeout;
mod handle_proximity_parameter;
mod handle_osc_message;

// TODO: clean up and refactor to avoid using as many .clone() statements

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    let mut config = config::load_config();

    // Setup Start / Stop of Terminator
    let running = Arc::new(AtomicBool::new(false));

    // Rx/Tx Socket Setup
    let mut rx_socket = giggletech_osc::setup_rx_socket(&config.port_rx).await?;

    // Timeout
    for ip in config.headpat_device_uris.clone() {
        let headpat_device_ip_clone = ip.clone();
        let timeout = config.timeout_setting.clone();
        tokio::spawn(async move {
            osc_timeout(&headpat_device_ip_clone, timeout).await.unwrap();
        });
    }
    // Listen for OSC Packets
    loop {
        tokio::select! {
            res = rx_socket.next() => {
                handle_osc_message::handle_osc_message(res, &mut config, running.clone()).await?;
            },
            //TODO add twitch integration input
        }
    }
}
