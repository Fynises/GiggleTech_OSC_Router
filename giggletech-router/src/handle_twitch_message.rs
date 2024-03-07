use std::{sync::{atomic::AtomicBool, Arc}, time::Instant};
use anyhow::Result;

use crate::{terminator, osc_timeout, giggletech_osc};

pub(crate) async fn handle_twitch_message(
    running: Arc<AtomicBool>,
    device_ip: &Arc<String>,
    power: i32,
) -> Result<()> {
    terminator::stop(running.clone()).await?;

    {
        let mut device_last_signal_times = osc_timeout::DEVICE_LAST_SIGNAL_TIME.lock().unwrap();
        device_last_signal_times.insert(device_ip.to_string(), Instant::now());
    }
    
    giggletech_osc::send_data(&device_ip, power).await?;
    Ok(())
}