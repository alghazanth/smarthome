use crate::devices::SmartDeviceTrait;
use crate::error::{Result, SmartHomeError};
use crate::rooms::Room;

#[allow(unused)]
pub struct House {
    pub name: String,
    pub rooms: Vec<Room>,
}

#[allow(unused)]
impl House {
    pub fn add_room(&mut self, room: Room) -> Result<()> {
        if !self.rooms.contains(&room) {
            self.rooms.push(room);
            Ok(())
        } else {
            Err(SmartHomeError::AlreadyExistentRoom)
        }
    }

    pub fn remove_room(&mut self, name: String) -> Result<()> {
        if let Some(index) = self.rooms.iter().position(|room| room.get_name() == name) {
            self.rooms.remove(index);
            Ok(())
        } else {
            Err(SmartHomeError::NonExistentRoom)
        }
    }

    pub fn list_rooms(&self) -> Option<String> {
        let list: String = self
            .rooms
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.name + "\n");
        match list.len() {
            x if x > 0 => Some(list),
            _ => None,
        }
    }

    pub fn get_report(&self) -> Option<String> {
        let report: String = self
            .rooms
            .iter()
            .map(|x| {
                x.devices
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &arg.get_status() + "\n")
            })
            .collect();
        match report.len() {
            x if x > 0 => Some(report),
            _ => None,
        }
    }
}
