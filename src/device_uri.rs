#[derive(Debug, Clone)]
pub enum DeviceType {
    Motor,
    Spark
}

impl DeviceType {
    pub fn parse(input: &str) -> Self {
        match input {
            "motor" => Self::Motor,
            "spark" => Self::Spark,
            _ => panic!("unknown device type found")
        }
    }

    pub fn as_address(&self) -> String {
        match self {
            DeviceType::Motor => String::from("/avatar/parameters/motor"),
            DeviceType::Spark => String::from("/motor"),
        }
    }
}

#[derive(Debug)]
pub struct DeviceUri {
    pub uri: String,
    pub device_type: DeviceType,
}
