use std::sync::Arc;
use actor_lite::handle::ActorHandle;
use anyhow::Result;
use url::Url;
use crate::{twitch_integration::ws_client::TwitchWsClient, message_queue::message::QueueMessage};

/// the websocket session that is used to communicate with the server
pub struct TwitchSocket {
    _handle: ezsockets::Client<TwitchWsClient>
}

impl TwitchSocket {
    pub async fn new(queue: Arc<ActorHandle<QueueMessage>>, url_string: String) -> Result<Self> {
        let url = Url::parse(url_string.as_str())?;
        let config = ezsockets::ClientConfig::new(url)
            .max_initial_connect_attempts(3)
            .max_reconnect_attempts(3);
        let (handle, future) = ezsockets::connect(|_c| TwitchWsClient::new(queue), config).await;
        println!("connected to twitch integration");
        tokio::spawn(async move {
            if let Err(e) = future.await {
                println!("error occurred in websocket session: {e:#?}");
                println!("twitch integration will not function");
            }
        });
        
        Ok(Self { _handle: handle })
    }

    pub fn _close(&self) -> Result<()> {
        self._handle.close(None)?;
        Ok(())
    }
}
