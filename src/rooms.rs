use crate::devices::*;
use crate::error::*;

#[allow(unused)]
pub struct Room<'a> {
    pub name: &'a str,
    pub devices: Vec<SmartDevice<'a>>,
}

#[allow(unused)]
impl<'a> Room<'a> {
    fn get_name(&self) -> &str {
        self.name
    }

    pub fn add_device(&mut self, device: SmartDevice<'a>) -> Result<()> {
        if !self.devices.contains(&device) {
            self.devices.push(device);
            Ok(())
        } else {
            Err(Error::AlreadyExistentDevice)
        }
    }

    pub fn remove_device(&mut self, device: SmartDevice<'a>) -> Result<()> {
        if self.devices.contains(&device) {
            self.devices.retain(|x| *x != device);
            Ok(())
        } else {
            Err(Error::NonExistentDevice)
        }
    }

    pub fn list_devices(&self) -> String {
        self.devices
            .iter()
            .fold(String::new(), |acc, arg| acc + arg.get_name())
    }
}

impl PartialEq for Room<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
