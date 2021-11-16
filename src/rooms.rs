#[allow(unused)]
struct Room<'a> {
    name: &'a str,
    devices: Vec<SmartDevice<'a>>,
}

#[allow(unused)]
impl<'a> Room<'a> {
    fn get_name(&self) -> &str {
        self.name
    }

    fn add_device(&mut self, device: SmartDevice<'a>) {
        if !self.devices.contains(&device) {
            self.devices.push(device);
        } else {
            println!("Device {} already exists in this house!", device.get_name());
        }
    }

    fn remove_device(&mut self, device: SmartDevice<'a>) {
        if !self.devices.contains(&device) {
            self.devices.retain(|x| *x != device);
        } else {
            println!("Device {} doesn't exist in this house!", device.get_name());
        }
    }

    fn list_devices(&self) {
        println!(
            "{}",
            self.devices
                .iter()
                .fold(String::new(), |acc, arg| acc + arg.get_name())
        );
    }
}

impl PartialEq for Room<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
