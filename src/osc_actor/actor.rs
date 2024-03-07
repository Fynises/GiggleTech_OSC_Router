use crate::{config::Config, main_actor::handle::MainActorHandle};
use anyhow::Result;
use async_osc::{prelude::OscMessageExt, Error, OscMessage, OscPacket, OscSocket, OscType};
use futures_util::StreamExt;
use std::{net::SocketAddr, sync::Arc};

type OscRxPacket = Result<(OscPacket, SocketAddr), Error>;

pub struct OscActor {
    rx_socket: OscSocket,
    main_actor: Arc<MainActorHandle>,
    config: Arc<Config>,
}

impl OscActor {
    pub async fn new(main_actor: Arc<MainActorHandle>, config: Arc<Config>) -> Self {
        let address = format!("127.0.0.1:{}", config.port_rx);
        let rx_socket = OscSocket::bind(address).await
            .expect("error occurred connecting to OSC");
        Self {
            rx_socket,
            main_actor,
            config,
        }
    }

    pub async fn run(&mut self) {
        while let Some(packet) = self.rx_socket.next().await {
            match self.handle_packet(packet).await {
                Ok(_) => (),
                Err(e) => log::error!("error occurred handling Osc packet {}", e),
            }
        }
    }

    async fn handle_packet(&self, rx_packet: OscRxPacket) -> Result<()> {
        let (packet, _peer_addr) = rx_packet?;
        match packet {
            OscPacket::Message(msg) => self.handle_osc_message(msg).await?,
            OscPacket::Bundle(_) => (),
        }
        Ok(())
    }

    async fn handle_osc_message(&self, msg: OscMessage) -> Result<()> {
        let (address, osc_value) = msg.as_tuple();
        let value = match osc_value.first().unwrap_or(&OscType::Nil).clone().float() {
            Some(v) => v,
            None => {
                log::warn!("error getting osc value from packet");
                return Ok(());
            }
        };

        let mut max_speed = self.config.max_speed_float;

        if address == self.config.max_speed_parameter_address {
            max_speed = value.max(self.config.max_speed_low_limit)
        } else {
            self.handle_proximity(address.to_string(), value, max_speed)
                .await?;
        }
        Ok(())
    }

    async fn handle_proximity(
        &self,
        device_id: String,
        value: f32,
        max_speed: f32,
    ) -> Result<()> {
        if value == 0.0 {
            log::info!("stopping...");
            for _ in 0..5 {
                self.main_actor.sender().send((device_id.clone(), 0i32))?;
            }
        } else {
            let processed_value = process_pat(
                value,
                max_speed,
                self.config.min_speed_float,
                self.config.speed_scale_float,
                &device_id
            );
            self.main_actor.sender().send((device_id, processed_value))?
        }
        Ok(())
    }
}

// copied from original data_processing.rs

fn proximity_graph(proximity_signal: f32) -> String {
    let num_dashes = (proximity_signal * 10.0) as usize;
    let graph = "-".repeat(num_dashes) + ">";
    graph
}

const MOTOR_SPEED_SCALE: f32 = 0.66; // Overvolt   Here, OEM config 0.66 going higher than this value will reduce your vibrator motor life

fn process_pat(
    proximity_signal: f32,
    max_speed: f32,
    min_speed: f32,
    speed_scale: f32,
    proximity_parameter: &String,
) -> i32 {
    let graph_str = proximity_graph(proximity_signal);
    let headpat_tx = (((max_speed - min_speed) * proximity_signal + min_speed)
        * MOTOR_SPEED_SCALE
        * speed_scale
        * 255.0)
        .round() as i32;
    let proximity_signal = format!("{:.2}", proximity_signal);
    log::info!(
        "{} Prox: {:5} Motor Tx: {:3} |{:11}|",
        proximity_parameter.trim_start_matches("/avatar/parameters/"),
        proximity_signal,
        headpat_tx,
        graph_str
    );

    headpat_tx
}
