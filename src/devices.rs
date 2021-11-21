use crate::error::{Error, Result};

#[allow(unused)]
pub struct Socket {
    pub name: String,
    pub description: String,
    pub status: String,
    pub consumption: i32,
}

#[allow(unused)]
pub struct Thermometer {
    pub name: String,
    pub description: String,
    pub temperature: i32,
}

#[allow(unused)]
pub enum SmartDevice {
    Socket(Socket),
    Thermometer(Thermometer),
}

impl PartialEq for SmartDevice {
    fn eq(&self, other: &Self) -> bool {
        use crate::devices::SmartDevice::Socket;
        use crate::devices::SmartDevice::Thermometer;
        match (self, other) {
            (Socket(a), Socket(b)) => a.name == b.name,
            (Thermometer(a), Thermometer(b)) => a.name == b.name,
            _ => false,
        }
    }
}

pub trait SmartDeviceTrait {
    fn get_name(&self) -> String;

    fn get_status(&self) -> String;
}

impl SmartDeviceTrait for crate::devices::SmartDevice {
    fn get_name(&self) -> String {
        use crate::devices::SmartDevice::Socket;
        use crate::devices::SmartDevice::Thermometer;
        match self {
            Socket(s) => s.name.clone(),
            Thermometer(t) => t.name.clone(),
        }
    }

    fn get_status(&self) -> String {
        match self {
            SmartDevice::Socket(s) => format!(
                "Socket {} is {} and consumption is {} W",
                s.name, s.status, s.consumption
            ),
            SmartDevice::Thermometer(t) => format!(
                "Thermometer {} indicates temperature of {} C",
                t.name, t.temperature
            ),
        }
    }
}

#[allow(unused)]
impl Socket {
    pub fn switch(&mut self) -> Result<()> {
        match self.status.as_ref() {
            "enabled" => {
                self.status = "disabled".to_string();
                Ok(())
            }
            "disabled" => {
                self.status = "enabled".to_string();
                Ok(())
            }
            _ => Err(Error::BrokenSocket),
        }
    }

    pub fn get_consumption(&self) -> i32 {
        self.consumption
    }
}

#[allow(unused)]
impl Thermometer {
    pub fn get_temperature(&self) -> i32 {
        self.temperature
    }
}
