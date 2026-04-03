use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};

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

pub struct Map {
    pub rooms: HashMap<RoomId, Room>,
}

impl Map {
    pub fn new() -> Self {
        Map {
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        self.rooms.insert(room.id, room);
    }

    pub fn get_room(&self, id: RoomId) -> Option<&Room> {
        self.rooms.get(&id)
    }
}

pub struct Player {
    pub current_room: RoomId,
}

impl Player {
    pub fn new(starting_room: RoomId) -> Self {
        Player {
            current_room: starting_room,
        }
    }

    pub fn look(&self, map: &Map) {
        match map.get_room(self.current_room) {
            Some(room) => {
                println!("{}", room.name.blue());
                println!("{}", room.description.cyan());
                print!("Exits: ");
                room.exits.keys().for_each(|r| print!("{r} "));
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
}

fn main() {
    println!("Welcome to Zorlike!");

    let mut room1 = Room::new(
        0,
        "Dungeon Entrance",
        "You stand in front of high dungeon. There is a door in front of you.",
    );
    let room2 = Room::new(
        1,
        "Dark Hallway",
        "You are in a dark hallway full of spiderwebs",
    );
    room1.add_exit("north", 1);

    let mut map = Map::new();

    map.add_room(room1);
    map.add_room(room2);

    let mut player = Player::new(0);

    player.look(&map);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let words: Vec<&str> = input.trim().split_whitespace().collect();

        if words.is_empty() {
            continue;
        }

        let verb = words[0];

        match verb {
            "quit" | "exit" => {
                println!("Goodbye!");
                break;
            }
            "look" => {
                player.look(&map);
            }
            "go" => {
                if words.len() > 1 {
                    player.go(words[1], &map);
                    player.look(&map);
                } else {
                    println!("Go where?")
                }
            }
            _ => println!("I don't understand that command."),
        }
    }
}
