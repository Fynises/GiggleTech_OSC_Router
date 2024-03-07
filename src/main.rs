use std::sync::Arc;
use crate::{config::Config, device_socket::DeviceSocket, main_actor::handle::MainActorHandle, osc_actor::actor::OscActor};

mod main_actor;
mod osc_actor;
mod config;
mod device_socket;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("");
    println!("  ██████  ██  ██████   ██████  ██      ███████     ████████ ███████  ██████ ██   ██ ");
    println!(" ██       ██ ██       ██       ██      ██             ██    ██      ██      ██   ██ ");
    println!(" ██   ███ ██ ██   ███ ██   ███ ██      █████          ██    █████   ██      ███████ ");
    println!(" ██    ██ ██ ██    ██ ██    ██ ██      ██             ██    ██      ██      ██   ██ ");
    println!("  ██████  ██  ██████   ██████  ███████ ███████        ██    ███████  ██████ ██   ██ ");
    println!("");
    println!(" █▀█ █▀ █▀▀   █▀█ █▀█ █ █ ▀█▀ █▀▀ █▀█");
    println!(" █▄█ ▄█ █▄▄   █▀▄ █▄█ █▄█  █  ██▄ █▀▄");

    // load the config, as Arc
    let application_config = Arc::new(Config::load());

    println!("\n");
    println!("\n");
    println!(" Device Maps");
    for (i, parameter) in application_config.proximity_parameters_multi.iter().enumerate() {
        println!(" {} => {}", parameter.trim_start_matches("/avatar/parameters/"), application_config.headpat_device_uris[i]);
    }

    println!("\n Listening for OSC on port: {}", application_config.port_rx);
    println!("\n Vibration Configuration");
    println!(" Min Speed: {}%", application_config.min_speed_float);
    println!(" Max Speed: {:?}%", application_config.max_speed_float * 100.0);
    println!(" Scale Factor: {}%", application_config.speed_scale_float);
    println!(" Timeout: {}s", application_config.timeout_setting);
    println!("\nWaiting for pats...");



    let devices = DeviceSocket::from_config(application_config.clone()).await;
    let main_actor = Arc::new(MainActorHandle::new(devices));

    let mut osc_actor = OscActor::new(main_actor.clone(), application_config.clone()).await;
    tokio::spawn(async move {
        osc_actor.run().await;
    }).await.unwrap();


}
