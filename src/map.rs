use crate::room::{Room, RoomId};
use std::collections::HashMap;

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

    pub fn get_room_mut(&mut self, id: RoomId) -> Option<&mut Room> {
        self.rooms.get_mut(&id)
    }
}
