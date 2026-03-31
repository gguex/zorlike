use std::collections::HashMap;

pub type RoomId = u32;

#[derive(Debug)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, RoomId>,
}

impl Room {
    pub fn new(id: RoomId, name: &str, description: &str) -> Self {
        Room {
            id,
            name: String::from(name),
            description: String::from(description),
            exits: HashMap::new(),
        }
    }

    pub fn add_exit(&mut self, direction: &str, target_id: RoomId) {
        self.exits.insert(String::from(direction), target_id);
    }
}

fn main() {
    println!("Welcome to Zorlike!");

    let mut room1 = Room::new(0, "Dungeon Entrance", "");
    let _room2 = Room::new(1, "Dark Hallway", "");
    room1.add_exit("north", 1);

    println!("{:#?}", room1);
}
