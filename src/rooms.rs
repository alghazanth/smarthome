use crate::devices::{SmartDevice, SmartDeviceTrait};
use crate::error::{Error, Result};

#[allow(unused)]
pub struct Room {
    pub name: String,
    pub devices: Vec<SmartDevice>,
}

#[allow(unused)]
impl Room {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_device(&mut self, device: SmartDevice) -> Result<()> {
        if !self.devices.contains(&device) {
            self.devices.push(device);
            Ok(())
        } else {
            Err(Error::AlreadyExistentDevice)
        }
    }

    pub fn remove_device(&mut self, name: String) -> Result<()> {
        if let Some(index) = self
            .devices
            .iter()
            .position(|device| device.get_name() == name)
        {
            self.devices.remove(index);
            Ok(())
        } else {
            Err(Error::NonExistentDevice)
        }
    }

    pub fn get_device(&self, name: String) -> Option<&SmartDevice> {
        self.devices
            .iter()
            .find(|&device| device.get_name() == name)
    }

    pub fn list_devices(&self) -> String {
        self.devices
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.get_name() + "\n")
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
