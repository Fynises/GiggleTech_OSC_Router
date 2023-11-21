// config.rs

use configparser::ini::Ini;
use std::{net::IpAddr};

// Banner
fn banner_txt(){
    // https://fsymbols.com/generators/carty/
    println!("");
    println!("  ██████  ██  ██████   ██████  ██      ███████     ████████ ███████  ██████ ██   ██ ");
    println!(" ██       ██ ██       ██       ██      ██             ██    ██      ██      ██   ██ ");
    println!(" ██   ███ ██ ██   ███ ██   ███ ██      █████          ██    █████   ██      ███████ ");
    println!(" ██    ██ ██ ██    ██ ██    ██ ██      ██             ██    ██      ██      ██   ██ ");
    println!("  ██████  ██  ██████   ██████  ███████ ███████        ██    ███████  ██████ ██   ██ ");
    println!("");
    println!(" █▀█ █▀ █▀▀   █▀█ █▀█ █ █ ▀█▀ █▀▀ █▀█");
    println!(" █▄█ ▄█ █▄▄   █▀▄ █▄█ █▄█  █  ██▄ █▀▄");
                                                                                
}

#[derive(Clone, Debug)]
pub struct GiggleTechConfig {
    pub twitch_integration_url: String,
    pub headpat_device_uris: Vec<String>,
    pub min_speed_float: f32,
    pub max_speed_float: f32,
    pub speed_scale_float: f32,
    pub port_rx: String,
    pub proximity_parameters_multi: Vec<String>,
    pub max_speed_parameter_address: String,
    pub max_speed_low_limit: f32,
    pub timeout_setting: u64,
}

pub(crate) fn load_config() -> GiggleTechConfig {
    let mut config = Ini::new();

    match config.load("./config.ini") {
        Err(why) => panic!("{}", why),
        Ok(_) => {}
    }
    
    // Check the format of the IP URIs
    let headpat_device_uris: Vec<String> = config.get("Setup", "device_ips")
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string()) // convert &str to String
        .filter_map(|s| {
            match s.parse::<IpAddr>() {
                Ok(_) => Some(s),
                Err(_) => {
                    println!("Invalid IP address format: {}", s);
                    None
                }
            }
        })
        .collect();
    if headpat_device_uris.is_empty() {
        eprintln!("Error: no device URIs specified in config file");
        // handle error here, e.g. return early from the function or exit the program
    }

    let proximity_parameters_multi: Vec<String> = config
    .get("Setup", "proximity_parameters_multi")
    .unwrap()
    .split_whitespace()
    .map(|s| format!("/avatar/parameters/{}", s))
    .collect();

    
    if headpat_device_uris.len() != proximity_parameters_multi.len() {
        eprintln!("Error: number of device URIs does not match number of proximity parameters");
        // handle error here, e.g. return early from the function or exit the program
    }

    const MAX_SPEED_LOW_LIMIT_CONST: f32 = 0.05;

    let min_speed = config.get("Config", "min_speed").unwrap();
    let min_speed_float = min_speed.parse::<f32>().unwrap() / 100.0;
    
    let max_speed = config.get("Config", "max_speed").unwrap().parse::<f32>().unwrap() / 100.0; 
    let max_speed_low_limit = MAX_SPEED_LOW_LIMIT_CONST;
    let max_speed_float = max_speed.max(max_speed_low_limit);
    
    let speed_scale = config.get("Config", "max_speed_scale").unwrap();
    let speed_scale_float = speed_scale.parse::<f32>().unwrap() / 100.0;
    
    let port_rx = config.get("Setup", "port_rx").unwrap();
    
    let timeout_str = config.get("Config", "timeout").unwrap();
    let timeout_setting = timeout_str.parse::<u64>().unwrap_or(0);
    
    let max_speed_parameter_address = format!("/avatar/parameters/{}", config.get("Setup", "max_speed_parameter").unwrap_or_else(|| "/avatar/parameters/max_speed".into()));

    println!("\n");
    banner_txt();
    println!("\n");
    println!(" Device Maps");
    for (i, parameter) in proximity_parameters_multi.iter().enumerate() {
        println!(" {} => {}", parameter.trim_start_matches("/avatar/parameters/"), headpat_device_uris[i]);
    }

    println!("\n Listening for OSC on port: {}", port_rx);
    println!("\n Vibration Configuration");
    println!(" Min Speed: {}%", min_speed);
    println!(" Max Speed: {:?}%", max_speed_float * 100.0);
    println!(" Scale Factor: {}%", speed_scale);
    println!(" Timeout: {}s", timeout_setting);
    println!("\nWaiting for pats...");

    let twitch_url = config.get("Setup", "twitch_integration_url").unwrap_or(String::from(""));
    println!("twitch url: {}", twitch_url);

    GiggleTechConfig {
        twitch_integration_url: twitch_url,
        headpat_device_uris,
        min_speed_float,
        max_speed_float,
        speed_scale_float,
        port_rx,
        proximity_parameters_multi,
        max_speed_parameter_address,
        max_speed_low_limit,
        timeout_setting,
    }
}



