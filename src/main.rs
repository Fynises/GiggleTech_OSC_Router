use std::sync::Arc;
use crate::{config::Config, device_socket::DeviceSocket, main_actor::handle::MainActorHandle, osc_actor::actor::OscActor};

mod main_actor;
mod osc_actor;
mod config;
mod device_socket;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt::init();

    log::info!("starting giggletech router");

    // load the config, as Arc
    let application_config = Arc::new(Config::load());
    let devices = DeviceSocket::from_config(application_config.clone()).await;
    let main_actor = Arc::new(MainActorHandle::new(devices));

    let mut osc_actor = OscActor::new(main_actor.clone(), application_config.clone()).await;
    tokio::spawn(async move {
        osc_actor.run().await;
    }).await.unwrap();


}
