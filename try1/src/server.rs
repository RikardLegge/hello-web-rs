use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use connection::Connection;

pub struct Player {
    pub name: String,
    pub con: Rc<Connection>
}

impl Player {
    pub fn new(name: &str, con: Rc<Connection>) ->Player {
        Player {
            name: String::from(name),
            con
        }
    }
}

pub struct Room {
    pub players: Vec<Weak<RefCell<Player>>>,
    con: Rc<Connection>
}

pub struct Server {
    players: Vec<Weak<RefCell<Player>>>,
    rooms: HashMap<String, Weak<RefCell<Room>>>
}

impl Server {
    pub fn new() -> Server {
        let players = Vec::new();
        let rooms = HashMap::new();
        Server { players, rooms }
    }

    pub fn add_player(&mut self, player: &Rc<RefCell<Player>>) {
        println!("New player connected: {:?}", player.borrow().name);
        let week_player = Rc::downgrade(player);
        self.players.push(week_player);
    }

    pub fn get_room_by_name(&self, room_name: &str) -> Result<Rc<RefCell<Room>>, String> {
        let room = self.rooms.get(room_name).ok_or(String::from("Room missing"))?;
        room.upgrade().ok_or(String::from("Room invalid"))
    }

    pub fn add_player_to_room(&mut self, player: Rc<RefCell<Player>>, room: Rc<RefCell<Room>>) {
        let player = Rc::downgrade(&player);
        room.borrow_mut().players.push(player);
    }
}

