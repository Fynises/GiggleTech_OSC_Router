use std::sync::Arc;
use actor_lite::handle::ActorHandle;
use ezsockets::Error as Error;
use crate::message_queue::message::QueueMessage;
use super::message::WebSocketMessage;

pub struct TwitchWsClient {
    queue: Arc<ActorHandle<QueueMessage>>
}

impl TwitchWsClient {
    pub fn new(queue: Arc<ActorHandle<QueueMessage>>) -> Self {
        Self { queue }
    }
}

#[async_trait::async_trait]
impl ezsockets::ClientExt for TwitchWsClient {
    type Call = ();

    async fn on_text(&mut self, text: String) -> Result<(), Error> {
        println!("received text: {}", text);
        let twitch_message = WebSocketMessage::parse(text.as_str())?;
        println!("deserialized as: {twitch_message:#?}");
        self.queue.send(QueueMessage::FromTwitch(twitch_message))?;
        Ok(())
    }

    async fn on_binary(&mut self, _bytes: Vec<u8>) -> Result<(), Error> {
        println!("unhandled binary received from websocket");
        Ok(())
    }

    async fn on_call(&mut self, _call: Self::Call) -> Result<(), Error> {
        println!("unhandled on_call received from websocket");
        Ok(())
    }
}
