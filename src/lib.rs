mod devices;
mod error;
mod house;
mod rooms;

#[cfg(test)]
mod tests {
    use crate::devices::{SmartDevice, SmartDeviceTrait, Socket, Thermometer};
    use crate::error::Error;
    use crate::house::House;
    use crate::rooms::Room;

    fn new_socket(name: String) -> Socket {
        Socket {
            name,
            description: "description".to_string(),
            status: "disabled".to_string(),
            consumption: 0,
        }
    }

    fn new_thermometer(name: String) -> Thermometer {
        Thermometer {
            name,
            description: "description".to_string(),
            temperature: 0,
        }
    }

    fn new_room(name: String) -> Room {
        Room {
            name,
            devices: vec![],
        }
    }

    fn new_house(name: String) -> House {
        House {
            name,
            rooms: vec![],
        }
    }

    #[test]
    fn add_device_to_room() {
        let mut room = new_room("bedroom".to_string());
        let socket = new_socket("socket".to_string());
        let thermometer = new_thermometer("thermometer".to_string());
        room.add_device(SmartDevice::Socket(socket))
            .expect("Can't add device");
        room.add_device(SmartDevice::Thermometer(thermometer))
            .expect("Can't add device");
        assert_eq!(room.list_devices(), "socket\nthermometer\n");
    }

    #[test]
    fn add_duplicate_device_to_room() {
        let mut room = new_room("bedroom".to_string());
        let thermometer = new_thermometer("thermometer".to_string());
        let thermometer_2 = new_thermometer("thermometer".to_string());
        room.add_device(SmartDevice::Thermometer(thermometer))
            .expect("Can't add device");
        assert_eq!(
            room.add_device(SmartDevice::Thermometer(thermometer_2)),
            Err(Error::AlreadyExistentDevice)
        );
    }

    #[test]
    fn list_devices_in_room() {
        let mut room = new_room("bedroom".to_string());
        let socket_1 = new_socket("socket_1".to_string());
        let socket_2 = new_socket("socket_2".to_string());
        let thermometer_1 = new_thermometer("thermometer_1".to_string());
        let thermometer_2 = new_thermometer("thermometer_2".to_string());
        room.add_device(SmartDevice::Socket(socket_1))
            .expect("Can't add device");
        room.add_device(SmartDevice::Socket(socket_2))
            .expect("Can't add device");
        room.add_device(SmartDevice::Thermometer(thermometer_1))
            .expect("Can't add device");
        room.add_device(SmartDevice::Thermometer(thermometer_2))
            .expect("Can't add device");
        assert_eq!(
            room.list_devices(),
            "socket_1\nsocket_2\nthermometer_1\nthermometer_2\n"
        );
    }

    #[test]
    fn remove_device_from_room() {
        let mut room = new_room("bedroom".to_string());
        let socket_1 = new_socket("socket_1".to_string());
        let socket_2 = new_socket("socket_2".to_string());
        let thermometer_1 = new_thermometer("thermometer_1".to_string());
        let thermometer_2 = new_thermometer("thermometer_2".to_string());
        room.add_device(SmartDevice::Socket(socket_1))
            .expect("Can't add device");
        room.add_device(SmartDevice::Socket(socket_2))
            .expect("Can't add device");
        room.add_device(SmartDevice::Thermometer(thermometer_1))
            .expect("Can't add device");
        room.add_device(SmartDevice::Thermometer(thermometer_2))
            .expect("Can't add device");
        room.remove_device(String::from("thermometer_1"))
            .expect("Can't remove device");
        assert_eq!(room.list_devices(), "socket_1\nsocket_2\nthermometer_2\n");
    }

    #[test]
    fn add_rooms_to_house() {
        let mut house = new_house("house".to_string());
        let room = new_room("bedroom".to_string());
        let room_2 = new_room("kitchen".to_string());
        house.add_room(room).expect("Can't add the room");
        house.add_room(room_2).expect("Can't add the room");
        assert_eq!(house.list_rooms(), "bedroom\nkitchen\n");
    }

    #[test]
    fn add_duplicate_rooms_to_house() {
        let mut house = new_house("house".to_string());
        let room = new_room("bedroom".to_string());
        let room_2 = new_room("kitchen".to_string());
        let room_3 = new_room("bedroom".to_string());
        house.add_room(room).expect("Can't add the room");
        house.add_room(room_2).expect("Can't add the room");
        assert_eq!(house.add_room(room_3), Err(Error::AlreadyExistentRoom));
    }

    #[test]
    fn remove_room_from_house() {
        let mut house = new_house("house".to_string());
        let room = new_room("bedroom".to_string());
        let room_2 = new_room("kitchen".to_string());
        let room_3 = new_room("hallway".to_string());
        house.add_room(room).expect("Can't add the room");
        house.add_room(room_2).expect("Can't add the room");
        house.add_room(room_3).expect("Can't add the room");
        house
            .remove_room(String::from("kitchen"))
            .expect("Can't remove room");
        assert_eq!(house.list_rooms(), "bedroom\nhallway\n");
    }

    #[test]
    fn list_rooms_in_house() {
        let mut house = new_house("house".to_string());
        let room = new_room("bedroom".to_string());
        let room_2 = new_room("kitchen".to_string());
        let room_3 = new_room("hallway".to_string());
        house.add_room(room).expect("Can't add the room");
        house.add_room(room_2).expect("Can't add the room");
        house.add_room(room_3).expect("Can't add the room");
        assert_eq!(house.list_rooms(), "bedroom\nkitchen\nhallway\n");
    }

    #[test]
    fn get_full_report() {
        let mut house = new_house("house".to_string());
        let mut room = new_room("bedroom".to_string());
        let mut room_2 = new_room("kitchen".to_string());
        let mut room_3 = new_room("hallway".to_string());
        let socket_1 = new_socket("socket_1".to_string());
        let socket_2 = new_socket("socket_2".to_string());
        let thermometer_1 = new_thermometer("thermometer_1".to_string());
        let thermometer_2 = new_thermometer("thermometer_2".to_string());
        room.add_device(SmartDevice::Socket(socket_1))
            .expect("Can't add device");
        room_2
            .add_device(SmartDevice::Socket(socket_2))
            .expect("Can't add device");
        room_3
            .add_device(SmartDevice::Thermometer(thermometer_1))
            .expect("Can't add device");
        room.add_device(SmartDevice::Thermometer(thermometer_2))
            .expect("Can't add device");
        house.add_room(room).expect("Can't add the room");
        house.add_room(room_2).expect("Can't add the room");
        house.add_room(room_3).expect("Can't add the room");
        assert_eq!(house.get_report(), "Socket socket_1 is disabled and consumption is 0 W\nThermometer thermometer_2 indicates temperature of 0 C\nSocket socket_2 is disabled and consumption is 0 W\nThermometer thermometer_1 indicates temperature of 0 C\n");
    }

    #[test]
    fn switch_socket() {
        let mut socket = new_socket("socket".to_string());
        socket.switch().expect("Socket is broken");
        assert_eq!(
            SmartDevice::Socket(socket).get_status(),
            "Socket socket is enabled and consumption is 0 W"
        );
    }

    #[test]
    fn get_socket_load() {
        let socket = new_socket("socket".to_string());
        assert_eq!(socket.get_consumption(), 0);
    }

    #[test]
    fn get_temperature() {
        let thermometer = new_thermometer("thermometer".to_string());
        assert_eq!(thermometer.get_temperature(), 0);
    }
}
