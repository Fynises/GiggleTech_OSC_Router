// GiggleTech.io
// OSC Router
// by Sideways
// Based off OSC Async https://github.com/Frando/async-osc

use actor_lite::handle::ActorHandle;
use anyhow::Result;
use async_std::{stream::StreamExt, sync::Arc,};
use message_queue::{osc_environment::OscEnvironment, queue_handler::QueueHandler};
use twitch_integration::socket::TwitchSocket;
use std::sync::atomic::AtomicBool;

use crate::osc_timeout::osc_timeout;
mod data_processing;
mod config;
mod giggletech_osc;
mod terminator;
mod osc_timeout;
mod handle_proximity_parameter;
mod handle_osc_message;
mod handle_twitch_message;
mod twitch_integration;
mod message_queue;

// TODO: clean up and refactor to avoid using as many .clone() statements

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    let mut config = config::load_config();

    // Setup Start / Stop of Terminator
    let running = Arc::new(AtomicBool::new(true));

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

    let twitch_url = config.twitch_integration_url.clone();
    let osc_environment = OscEnvironment::new(config, running.clone());

    let message_queue = Arc::new(ActorHandle::new_async(|tx| QueueHandler::new(
        tx.clone(), 
        osc_environment, 
    )));

    let _twitch_client = match TwitchSocket::new(message_queue.clone(), twitch_url).await {
        Ok(v) => Some(v),
        Err(e) => {
            println!("error occurred establishing twitch integration connection: {e:#?}");
            None
        },
    };

    //Listen for OSC Packets
    loop {
        let osc_message = rx_socket.next().await;
        message_queue.send(message_queue::message::QueueMessage::FromOsc(osc_message))?
    }
}
