use std::collections::HashMap;

pub type RoomId = u32;

#[derive(Debug)]
pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub description: String,
    pub exits: HashMap<String, RoomId>,
    pub items: Vec<String>,
}

impl Room {
    pub fn new(id: RoomId, name: &str, description: &str) -> Self {
        Room {
            id,
            name: String::from(name),
            description: String::from(description),
            exits: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn add_exit(&mut self, direction: &str, target_id: RoomId) {
        self.exits.insert(String::from(direction), target_id);
    }

    pub fn add_item(&mut self, item: &str) {
        self.items.push(String::from(item));
    }
}
