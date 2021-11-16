#[allow(unused)]
struct House<'a> {
    name: String,
    rooms: Vec<Room<'a>>,
}

#[allow(unused)]
impl<'a> House<'a> {
    fn add_room(&mut self, room: Room<'a>) {
        if !self.rooms.contains(&room) {
            self.rooms.push(room);
        } else {
            println!("Room {} already exists in this house!", room.get_name());
        }
    }

    fn remove_room(&mut self, room: Room) {
        if self.rooms.contains(&room) {
            self.rooms.retain(|x| *x != room);
        } else {
            println!("Room {} doesn't exist in this house!", room.get_name());
        }
    }

    fn list_rooms(&self) {
        println!(
            "{}",
            self.rooms
                .iter()
                .fold(String::new(), |acc, arg| acc + arg.name)
        );
    }

    fn get_report(&self) {
        self.rooms
            .iter()
            .map(|x| x.devices.iter().map(|x| x.get_status()));
    }
}
