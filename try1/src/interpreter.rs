use serde_json::Value;

use std::rc::Rc;
use std::cell::RefCell;
use server::{Server, Player, Room};
use connection::Connection;

pub struct Interpreter {
    server: Server
}

pub struct InterpMsg {
    player: Option<Rc<RefCell<Player>>>,
    room: Option<Rc<RefCell<Room>>>,
    value: Value,
    pub con: Rc<Connection>
}

enum Role {
    Player,
    Room
}

pub enum ResultAction {
    None,
    UpdatePlayers(Rc<RefCell<Room>>)
}

impl InterpMsg {
    pub fn new(con: Rc<Connection>, player: Option<Rc<RefCell<Player>>>, room: Option<Rc<RefCell<Room>>>, value: Value) -> InterpMsg {
        InterpMsg {con, player, room, value}
    }

    pub fn get_str(&self, key: &str) -> Result<&str, String> {
        self.value[key].as_str().ok_or(format!("Message '{}' must be of type string", key))
    }

    pub fn get_role(&self) -> Result<Role, String> {
        match self.get_str("role")? {
            "room" => Ok(Role::Room),
            "player" => Ok(Role::Player),
            &_ => Err(String::from("Role not set"))
        }
    }
}

impl Interpreter {

    pub fn new(server: Server) -> Interpreter {
        Interpreter {server}
    }

    pub fn handle_message(&mut self, msg: &InterpMsg) -> Result<ResultAction, String> {
        let tp = msg.get_str("type")?;

        match tp {
            "connect" => self.connect(msg),
            "createRoom" => self.connect_room(msg),
            _ => Err(format!("Unknown message type {}", tp))
        }
    }

    fn connect(&mut self, msg: &InterpMsg) -> Result<ResultAction, String> {
        let role = msg.get_role()?;

        match role {
            Role::Room => self.connect_room(msg),
            Role::Player => self.connect_player(msg)
        }
    }

    fn connect_player(&mut self, msg: &InterpMsg) -> Result<ResultAction, String> {
        let name = msg.get_str("name")?;
        let player = Player::new(name, msg.con.clone());
        let player = Rc::new(RefCell::new(player));
        self.server.add_player(&player);

        let room_name = msg.get_str("room")?;
        let room = self.server.get_room_by_name(room_name)?;

        self.server.add_player_to_room(player, room.clone());
        Ok(ResultAction::UpdatePlayers(room))
    }

    fn connect_room(&mut self, msg: &InterpMsg) -> Result<ResultAction, String> {
        let name = msg.get_str("name")?;

        Ok(ResultAction::None)
    }
}