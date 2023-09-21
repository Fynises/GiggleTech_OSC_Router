// GiggleTech.io
// OSC Router
// by Sideways
// Based off OSC Async https://github.com/Frando/async-osc

use async_osc::{prelude::*, OscPacket, OscType, Result};
use async_std::{stream::StreamExt, sync::Arc,};
use std::sync::atomic::AtomicBool;

use crate::osc_timeout::osc_timeout;
mod data_processing;
mod config;
mod giggletech_osc;
mod terminator;
mod osc_timeout;
mod handle_proximity_parameter;

// TODO: clean up and refactor to avoid using as many .clone() statements

#[tokio::main]
async fn main() -> Result<()> {

    let mut config = config::load_config();

    // Setup Start / Stop of Terminator
    let running = Arc::new(AtomicBool::new(false));

    // Rx/Tx Socket Setup
    let mut rx_socket = giggletech_osc::setup_rx_socket(config.port_rx).await?;

    // Timeout
    for ip in config.headpat_device_uris.clone() {
        let headpat_device_ip_clone = ip.clone();
        let timeout = config.timeout_setting.clone();
        tokio::spawn(async move {
            osc_timeout(&headpat_device_ip_clone, timeout).await.unwrap();
        });
    }
    // Listen for OSC Packets
    while let Some(packet) = rx_socket.next().await {
        let (packet, _peer_addr) = packet?;

        // Filter OSC Signals
        match packet {
            OscPacket::Bundle(_) => {}
            OscPacket::Message(message) => {
                let (address, osc_value) = message.as_tuple();
                let value = match osc_value.first().unwrap_or(&OscType::Nil).clone().float() {
                    Some(v) => v,
                    None => continue,
                };

                // Max Speed Setting
                if address == config.max_speed_parameter_address {
                    data_processing::print_speed_limit(value);
                    config.max_speed_float = value.max(config.max_speed_low_limit.clone());
                } else {
                    let index = config.proximity_parameters_multi.iter().position(|a| *a == address);
                    if let Some(i) = index {
                        handle_proximity_parameter::handle_proximity_parameter(
                            running.clone(),
                            &Arc::new(config.headpat_device_uris[i].clone()), 
                            value, 
                            config.max_speed_float, 
                            config.min_speed_float, 
                            config.speed_scale_float, 
                            &config.proximity_parameters_multi[i]
                        ).await?;
                    } else {
                        log::error!("TODO:")
                    }
                }
            }
        }
    }
    Ok(())
}
