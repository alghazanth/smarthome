use crate::devices::{SmartDevice, Socket, Thermometer};
use crate::house::*;
use crate::rooms::*;

mod devices;
mod error;
mod house;
mod rooms;

fn main() {
    let socket = Socket {
        name: "socket",
        description: "description",
        status: "disabled",
        consumption: 0,
    };
    let thermometer = Thermometer {
        name: "thermometer",
        description: "description",
        temperature: 0,
    };

    let thermometer2 = Thermometer {
        name: "thermometer",
        description: "description",
        temperature: 0,
    };

    let mut room = Room {
        name: "some room",
        devices: vec![],
    };

    if let Err(e) = room.add_device(SmartDevice::Socket(socket)) {
        println!("Error: {}", e);
    };
    if let Err(e) = room.add_device(SmartDevice::Thermometer(thermometer)) {
        println!("Error: {}", e);
    };
    if let Err(e) = room.add_device(SmartDevice::Thermometer(thermometer2)) {
        println!("Error: {}", e);
    };

    let mut house = House {
        name: "some house",
        rooms: vec![],
    };
    if let Err(e) = house.add_room(room) {
        println!("Error: {}", e);
    };

    println!("{}", house.get_report());
    println!("{}", house.list_rooms());
}
