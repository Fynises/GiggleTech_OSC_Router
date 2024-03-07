use std::net::IpAddr;
use configparser::ini::Ini;

pub struct Config {
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

impl Config {
    /// loads the configuration, panics on error loading
    pub fn load() -> Self {
        let mut config = Ini::new();
        config.load("./config.ini").expect("error loading config.ini");
    
        Self {
            headpat_device_uris: Self::load_headpat_device_uris(&config),
            min_speed_float: config.get("Config", "min_speed").unwrap().parse::<f32>().unwrap(),
            max_speed_float: config.get("Config", "max_speed").unwrap().parse::<f32>().unwrap(),
            speed_scale_float: config.get("Config", "max_speed_scale").unwrap().parse::<f32>().unwrap(),
            port_rx: config.get("Setup", "port_rx").unwrap(),
            proximity_parameters_multi: Self::load_proximity_parameters_multi(&config),
            max_speed_parameter_address: Self::load_max_speed_parameters_address(&config),
            max_speed_low_limit: 0.05,
            timeout_setting: config.get("Config", "timeout").unwrap().parse::<u64>().unwrap_or(0),
        }
    }

    fn load_headpat_device_uris(config: &Ini) -> Vec<String> {
        let uris: Vec<String> = config.get("Setup", "device_ips")
            .expect("error loading config field Setup::device_ips")
            .split_whitespace()
            .map(|s| s.to_string())
            .filter_map(|s| {
                match s.parse::<IpAddr>() {
                    Ok(_) => Some(s),
                    Err(_) => {
                        log::error!("Invalid IP address format: {}", s);
                        None
                    }
                }
            }).collect();
        if uris.is_empty() {
            panic!("Error: no device URIs specified in config file")
        }

        uris
    }

    fn load_proximity_parameters_multi(config: &Ini) -> Vec<String> {
        config.get("Setup", "proximity_parameters_multi")
            .unwrap()
            .split_whitespace()
            .map(|s| format!("/avatar/parameters/{}", s))
            .collect()
    }

    fn load_max_speed_parameters_address(config: &Ini) -> String {
        format!("/avatar/parameters/{}", config.get("Setup", "max_speed_parameter").unwrap_or_else(|| "/avatar/parameters/max_speed".into()))
    }

}
