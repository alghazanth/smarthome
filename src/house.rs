use crate::devices::*;
use crate::error::*;
use crate::rooms::*;

#[allow(unused)]
pub struct House<'a> {
    pub name: &'a str,
    pub rooms: Vec<Room<'a>>,
}

#[allow(unused)]
impl<'a> House<'a> {
    pub fn add_room(&mut self, room: Room<'a>) -> Result<()> {
        if !self.rooms.contains(&room) {
            self.rooms.push(room);
            Ok(())
        } else {
            Err(Error::AlreadyExistentRoom)
        }
    }

    pub fn remove_room(&mut self, room: Room) -> Result<()> {
        if self.rooms.contains(&room) {
            self.rooms.retain(|x| *x != room);
            Ok(())
        } else {
            Err(Error::NonExistentRoom)
        }
    }

    pub fn list_rooms(&self) -> String {
        self.rooms
            .iter()
            .fold(String::new(), |acc, arg| acc + arg.name + "\n")
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
