use crate::devices::{SmartDevice, SmartDeviceTrait};
use crate::error::{Result, SmartHomeError};

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
            Err(SmartHomeError::AlreadyExistentDevice)
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
            Err(SmartHomeError::NonExistentDevice)
        }
    }

    pub fn get_device(&self, name: String) -> Option<&SmartDevice> {
        self.devices
            .iter()
            .find(|&device| device.get_name() == name)
    }

    pub fn list_devices(&self) -> Option<String> {
        let list: String = self
            .devices
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.get_name() + "\n");
        match list.len() {
            x if x > 0 => Some(list),
            _ => None,
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
