use std::{net::SocketAddr, sync::{Arc, atomic::AtomicBool}};
use anyhow::{Result, Context};
use async_osc::{OscPacket, Error, prelude::OscMessageExt, OscType};
use crate::handle_proximity_parameter;
use crate::data_processing;
use crate::config::GiggleTechConfig;

type OscRxPacket = Option<Result<(OscPacket, SocketAddr), Error>>;

pub async fn handle_osc_message(
    rx: OscRxPacket,
    config: &mut GiggleTechConfig,
    running_terminator: Arc<AtomicBool>,
) -> Result<()> {
    let (packet, _peer_addr) = rx.context("error receiving osc packet")??;
    
    match packet {
        OscPacket::Bundle(_) => todo!(),
        OscPacket::Message(message) => {
            let (address, osc_value) = message.as_tuple();
            let value = match osc_value.first().unwrap_or(&OscType::Nil).clone().float() {
                Some(v) => v,
                None => return Ok(()), //TODO: should this return something else?
            };
            
            if address == config.max_speed_parameter_address {
                data_processing::print_speed_limit(value);
                config.max_speed_float = value.max(config.max_speed_low_limit.clone());
            } else {
                let index = config.proximity_parameters_multi.iter().position(|a| *a == address);
                if let Some(i) = index {
                    handle_proximity_parameter::handle_proximity_parameter(
                        running_terminator.clone(),
                        &Arc::new(config.headpat_device_uris[i].clone()), 
                        value, 
                        config.max_speed_float, 
                        config.min_speed_float, 
                        config.speed_scale_float, 
                        &config.proximity_parameters_multi[i]
                    ).await?;
                } else {
                    log::error!("TODO")
                }
            }
        },
    }
    Ok(())
}
