pub mod map;
pub mod player;
pub mod room;

use map::Map;
use player::Player;
use room::Room;

use std::io::{self, Write};
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
    room1.add_item("key");

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
            "take" => {
                if words.len() > 1 {
                    player.take(words[1], &mut map);
                    player.look(&map);
                } else {
                    println!("Take what?")
                }
            }
            "inventory" => {
                println!("Inventory: {:?}", player.inventory);
            }
            _ => println!("I don't understand that command."),
        }
    }
}
