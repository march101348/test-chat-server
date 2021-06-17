use actix::prelude::*;
use std::collections::HashMap;

pub struct WsActor {
    sessions: HashMap<u32, Recipient<Message>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(u32)]
pub struct Connect {
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub id: u32,
    pub msg: String,
}

impl WsActor {
    pub fn new() -> WsActor {
        WsActor {
            sessions: HashMap::new(),
        }
    }

    fn send_message(&self, message: &str) {
        &self.sessions.iter().for_each(|(_, addr)| {
            let _ = addr.do_send(Message(message.to_owned()));
        });
    }
}

impl Actor for WsActor {
    type Context = Context<Self>;
}

impl Handler<Connect> for WsActor {
    type Result = u32;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let client_id = rand::random::<u32>();
        self.sessions.insert(client_id, msg.addr);
        self.send_message(&format!("{} connected!", client_id));
        client_id
    }
}

impl Handler<Disconnect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        let client_id = msg.id;
        self.send_message(&format!("{} disconnected...", client_id));
        self.sessions.remove(&client_id);
    }
}

impl Handler<ClientMessage> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Context<Self>) {
        use crate::repository::chat::{NewChat, create_chat};

        println!("{}", msg.msg);

        // isnert into chats DB
        // TODO: error handling
        let chat: NewChat = serde_json::from_str(&msg.msg)
            .expect("invalid struct for NewChat");
        let json = serde_json::to_string(&create_chat(chat))
            .expect("invalid struct for Chat");
        
        self.send_message(&json);
    }
}