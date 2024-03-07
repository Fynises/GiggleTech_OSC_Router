use std::collections::HashMap;
use tokio::sync::mpsc::{self, UnboundedSender};
use crate::device_socket::DeviceSocket;

use super::actor::MainActor;

pub struct MainActorHandle {
    sender: UnboundedSender<(String, i32)>,
}

impl MainActorHandle {
    pub fn new(sockets: HashMap<String, Vec<DeviceSocket>>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<(String, i32)>();
        let mut actor = MainActor::new(rx, sockets);
        tokio::spawn(async move { actor.run().await });
        Self { sender: tx }
    }

    pub fn sender(&self) -> &UnboundedSender<(String, i32)> {
        &self.sender
    }
}
