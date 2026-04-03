use crate::map::Map;
use crate::room::RoomId;
use colored::*;

pub struct Player {
    pub current_room: RoomId,
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(starting_room: RoomId) -> Self {
        Player {
            current_room: starting_room,
            inventory: Vec::new(),
        }
    }

    pub fn look(&self, map: &Map) {
        match map.get_room(self.current_room) {
            Some(room) => {
                println!("{}", room.name.blue());
                println!("{}", room.description.cyan());
                if !room.items.is_empty() {
                    print!("On the floor : ");
                    room.items.iter().for_each(|s| print!("{} ", s.yellow()));
                    println!("");
                }
                print!("Exits: ");
                room.exits.keys().for_each(|s| print!("{s} "));
                println!("");
            }
            None => {
                println!("Error: Room {} does not exist!", self.current_room)
            }
        }
    }

    pub fn go(&mut self, direction: &str, map: &Map) {
        let Some(current_room) = map.get_room(self.current_room) else {
            println!("You are in the void");
            return;
        };

        let Some(next_room) = current_room.exits.get(direction) else {
            println!("There is no such exits");
            return;
        };

        self.current_room = *next_room;
    }

    pub fn take(&mut self, item_name: &str, map: &mut Map) {
        let Some(room) = map.get_room_mut(self.current_room) else {
            return;
        };

        if let Some(index) = room.items.iter().position(|i| i == item_name) {
            println!("You take {}.", item_name.yellow());
            self.inventory.push(String::from(room.items.remove(index)));
        } else {
            println!("There is no {} here.", item_name);
        }
    }
}
