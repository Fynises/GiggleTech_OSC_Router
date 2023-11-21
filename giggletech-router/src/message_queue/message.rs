use crate::twitch_integration::message::WebSocketMessage;

use super::osc_type::OscRxPacket;

#[derive(Debug)]
pub enum QueueMessage {
    FromOsc(OscRxPacket),
    FromTwitch(WebSocketMessage),
    OnTwitchEnd,
}
