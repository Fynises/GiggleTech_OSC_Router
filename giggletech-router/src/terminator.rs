// terminator.rs

use anyhow::{anyhow, Result};
//use async_osc::Result;
use async_std::{task::{self},sync::Arc,};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::giggletech_osc;


 pub(crate) async fn start(running: Arc<AtomicBool>, device_ip: &Arc<String>) -> Result<()> {
    if running.load(Ordering::Relaxed) {
        return Err(anyhow!("worker is already running"));
    }
    let worker_running = running.clone();
    let worker_device_ip = device_ip.clone();
    let _ = task::spawn(async move {
        worker(worker_running, worker_device_ip).await.unwrap();
    });
    running.store(true, Ordering::SeqCst);
    Ok(())
}

async fn worker(running: Arc<AtomicBool>, device_ip: Arc<String>) -> Result<()> {
    while running.load(Ordering::Relaxed) {
        log::info!("Worker is running");
        giggletech_osc::send_data(&device_ip, 0i32).await?;
        task::sleep(Duration::from_secs(1)).await;
    }
    log::info!("Worker stopped");
    Ok(())
}

pub(crate) async fn stop(running: Arc<AtomicBool>) -> Result<()> {
    if !running.load(Ordering::Relaxed) {
        return Err(anyhow!("Worker is not running"));
    }
    running.store(false, Ordering::SeqCst);
    Ok(())
}