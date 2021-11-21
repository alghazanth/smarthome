use crate::devices::SmartDeviceTrait;
use crate::error::{Error, Result};
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
            Err(Error::AlreadyExistentRoom)
        }
    }

    pub fn remove_room(&mut self, name: String) -> Result<()> {
        if let Some(index) = self.rooms.iter().position(|room| room.get_name() == name) {
            self.rooms.remove(index);
            Ok(())
        } else {
            Err(Error::NonExistentRoom)
        }
    }

    pub fn list_rooms(&self) -> String {
        self.rooms
            .iter()
            .fold(String::new(), |acc, arg| acc + &arg.name + "\n")
    }

    pub fn get_report(&self) -> String {
        self.rooms
            .iter()
            .map(|x| {
                x.devices
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &arg.get_status() + "\n")
            })
            .collect()
    }
}
