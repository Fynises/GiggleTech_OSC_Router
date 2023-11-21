use std::sync::{atomic::AtomicBool, Arc};

use crate::config::GiggleTechConfig;

/// this struct is for the purpose of isolating\
/// the original OSC handling environment for use in new\
/// twitch integrated handler\
/// ideally this wouldn't be needed
pub struct OscEnvironment {
    pub config: GiggleTechConfig,
    pub running: Arc<AtomicBool>,
}

impl OscEnvironment {
    pub fn new(config: GiggleTechConfig, running: Arc<AtomicBool>) -> Self {
        Self {
            config, 
            running
        }
    }
}
