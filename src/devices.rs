#[allow(unused)]
struct Socket<'a> {
    name: &'a str,
    description: &'a str,
    status: &'a str,
    power_consumption: i32,
}

#[allow(unused)]
struct Thermometer<'a> {
    name: &'a str,
    description: &'a str,
    temperature: i32,
}

#[allow(unused)]
enum SmartDevice<'a> {
    Socket(Socket<'a>),
    Thermometer(Thermometer<'a>),
}

impl PartialEq for SmartDevice<'_> {
    fn eq(&self, other: &Self) -> bool {
        use crate::SmartDevice::Socket;
        use crate::SmartDevice::Thermometer;
        match (self, other) {
            (Socket(a), Socket(b)) => a.name == b.name,
            (Thermometer(a), Thermometer(b)) => a.name == b.name,
            _ => false,
        }
    }
}

trait SmartDeviceTrait {
    fn get_name(&self) -> &str;

    fn get_status(&self) -> String;
}

impl SmartDeviceTrait for crate::SmartDevice<'_> {
    fn get_name(&self) -> &str {
        use crate::SmartDevice::Socket;
        use crate::SmartDevice::Thermometer;
        match self {
            Socket(s) => s.name,
            Thermometer(t) => t.name,
        }
    }

    fn get_status(&self) -> String {
        match self {
            SmartDevice::Socket(s) => format!(
                "Socket {} is {} and consumption is {} W",
                s.name, s.status, s.power_consumption
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
        self.power_consumption
    }
}

#[allow(unused)]
impl Thermometer<'_> {
    fn get_temperature(&self) -> i32 {
        self.temperature
    }
}
