use serde::Deserialize;
use anyhow::Result;

/// message received from websocket
#[derive(Debug, Deserialize)]
pub struct WebSocketMessage {
    pub duration: usize, // duration provided in seconds
    pub magnitude: f64, // power as float value between 0.0 to 1.0
}

impl WebSocketMessage {
    /// parse from string
    pub fn parse(message: &str) -> Result<Self> {
        let data: Self = serde_json::from_str(message)?;
        Ok(data)
    }

    /// NOTE: hardcoded to be half of full strength for safety reasons
    pub fn get_percentage(&self) -> i32 {
        let fraction = self.magnitude / 2.0;
        (fraction * 250.0) as i32
    }
}
