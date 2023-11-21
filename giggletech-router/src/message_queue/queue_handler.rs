use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use actor_lite::error::Error;
use actor_lite::r#async::handler::Handler;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time;
use anyhow::Result;
use crate::{handle_osc_message, terminator};
use crate::handle_twitch_message::handle_twitch_message;
use crate::twitch_integration::message::WebSocketMessage;
use super::message::QueueMessage;
use super::osc_environment::OscEnvironment;
use super::osc_type::OscRxPacket;

pub struct QueueHandler {
    interrupt: bool, // when true, messages from OSC are ignored
    self_handler: UnboundedSender<QueueMessage>,
    osc_environment: OscEnvironment,
}

impl QueueHandler {
    pub fn new(
        tx: UnboundedSender<QueueMessage>,
        osc_environment: OscEnvironment,
    ) -> Self {
        Self {
            interrupt: false,
            self_handler: tx,
            osc_environment,
        }
    }

    async fn handle_osc(&mut self, osc: OscRxPacket) -> Result<(), Error> {
        if !self.interrupt {
            handle_osc_message::handle_osc_message(
                osc, 
                &mut self.osc_environment.config, 
                self.osc_environment.running.clone()
            ).await?;
        }
        Ok(())
    }

    fn handle_twitch(&mut self, msg: WebSocketMessage) {
        println!("initial interrupt state: {}", self.interrupt);
        if self.interrupt { return; }
        self.interrupt = true;

        let handler = self.self_handler.clone();
        let running = self.osc_environment.running.clone();
        let device_ip = self.osc_environment.config.headpat_device_uris[0].clone();

        let _ = tokio::spawn(async move {
            println!("device ip: {}", device_ip);
            match Self::run_twitch_message(msg, running, device_ip, handler).await {
                Ok(_) => (),
                Err(e) => println!("error occurred running twitch message: {e:#?}"),
            }
        });
    }

    async fn run_twitch_message(
        msg: WebSocketMessage,
        running: Arc<AtomicBool>,
        device_ip: String,
        handler: UnboundedSender<QueueMessage>,
    ) -> Result<()> {
        let mut sec_interval = time::interval(Duration::from_millis(500));
        sec_interval.tick().await;
        for _ in 0..(msg.duration * 2) {
            let _ = handle_twitch_message(
                running.clone(), 
                &Arc::new(device_ip.clone()), 
                msg.get_percentage()
            ).await?;
            sec_interval.tick().await;
        }
        let _ = handler.send(QueueMessage::OnTwitchEnd)?;
        terminator::start(running, &Arc::new(device_ip)).await?;
        Ok(())
    }

    fn handle_twitch_end(&mut self) {
        self.interrupt = false;
    }
}

#[async_trait::async_trait]
impl Handler<QueueMessage> for QueueHandler {
    async fn handle_message(&mut self, message: QueueMessage) -> Result<(), Error> {
        let _ = match message {
            QueueMessage::FromOsc(packet) => self.handle_osc(packet).await?,
            QueueMessage::FromTwitch(m) => self.handle_twitch(m),
            QueueMessage::OnTwitchEnd => self.handle_twitch_end(),
        };
        Ok(())
    }

    async fn on_error(&mut self, error: Error) {
        println!("error occurred in message queue: {error:#?}")
    }
}
