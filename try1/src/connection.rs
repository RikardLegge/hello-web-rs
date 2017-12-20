use ws;
use ws::{Handler, Sender, Message, CloseCode};
use serde_json;

use std::rc::Rc;
use std::cell::RefCell;

use interpreter::{Interpreter, InterpMsg, ResultAction};
use server::{Room, Player};

pub struct ConnectionHandler {
    connection: Rc<Connection>,
    interp: Rc<RefCell<Interpreter>>,

    player: Option<Rc<RefCell<Player>>>,
    room: Option<Rc<RefCell<Room>>>
}

pub struct Connection {
    out: Sender
}

impl Connection {
    pub fn send(&self, msg: &str) -> ws::Result<()> {
        self.out.send(msg)
    }
}


impl ConnectionHandler {
    pub fn new(out: Sender, interp: Rc<RefCell<Interpreter>>) -> ConnectionHandler {
        let player = None;
        let room = None;
        let connection = Rc::new(Connection {out});
        ConnectionHandler { connection, interp, player, room}
    }

    fn deserialize(&self, data: &String) -> Result<serde_json::Value, serde_json::Error> {
        serde_json::from_str(data)
    }
}

impl Handler for ConnectionHandler {

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        self.player = None;
        self.room = None;

        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            CloseCode::Abnormal => println!(
                "Closing handshake failed! Unable to obtain closing status from client."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let text = msg.into_text()?;
        match self.deserialize(&text) {
            Ok(value) => {
                println!("The client sent: {:?}", value);
                let player = self.player.clone();
                let room = self.room.clone();
                let msg = InterpMsg::new(self.connection.clone(), player, room, value);

                let mut interp = self.interp.borrow_mut();
                match interp.handle_message(&msg) {
                    Ok(act) =>{
                        match act {
                            ResultAction::UpdatePlayers(room) => {
                                let players = &room.borrow().players;

                                let player_names :Vec<serde_json::Value> = players.iter()
                                    .map(|player| {
                                        if let Some(player) = player.upgrade() {
                                            let val = player.borrow().name.clone();
                                            Some(val)
                                        } else {
                                            None
                                        }
                                    } )
                                    .filter(|name| name.is_some())
                                    .map(|name| serde_json::Value::String(name.unwrap())).collect();

                                let player_names = serde_json::Value::Array(player_names);

                                let msg = json!({
                                    "action": "updatePlayers",
                                    "players": player_names
                                }).to_string();

                                players.iter()
                                    .for_each(|player| {
                                        if let Some(player) = player.upgrade() {
                                            let con = player.borrow().con.clone();
                                            let con = con.as_ref();
                                            con.send(&msg[..]);
                                        }
                                    });
                            }
                            ResultAction::None => {}
                        }
                    },
                    Err(msg) => self.connection.send(&format!("{{err:\"{}\"}}", msg)[..])?,
                }
            },
            Err(msg) => {
                println!("The client sent invalid message: {:?}", msg);
                self.connection.send("{err: \"Invalid message\"}")?
            }
        }
        Ok(())
    }

    fn on_error(&mut self, err: ws::Error) {
        println!("The server encountered an error: {:?}", err);
    }
}