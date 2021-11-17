#[allow(unused)]
pub struct Socket<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub status: &'a str,
    pub consumption: i32,
}

#[allow(unused)]
pub struct Thermometer<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub temperature: i32,
}

#[allow(unused)]
pub enum SmartDevice<'a> {
    Socket(Socket<'a>),
    Thermometer(Thermometer<'a>),
}

impl PartialEq for SmartDevice<'_> {
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
    fn get_name(&self) -> &str;

    fn get_status(&self) -> String;
}

impl SmartDeviceTrait for crate::devices::SmartDevice<'_> {
    fn get_name(&self) -> &str {
        use crate::devices::SmartDevice::Socket;
        use crate::devices::SmartDevice::Thermometer;
        match self {
            Socket(s) => s.name,
            Thermometer(t) => t.name,
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
impl Socket<'_> {
    fn switch(&mut self) {
        match self.status {
            "enabled" => self.status = "disabled",
            "disabled" => self.status = "enabled",
            &_ => println!("Socket {} is probably broken", self.name),
        };
    }

    fn get_power_consumprion(&self) -> i32 {
        self.consumption
    }
}

#[allow(unused)]
impl Thermometer<'_> {
    fn get_temperature(&self) -> i32 {
        self.temperature
    }
}
