use diesel::prelude::*;

use super::super::domain::model::chat::*;
use super::super::domain::service::chat_service::*;
use crate::schema::chats;
use crate::utils::dsl::establish_connection;

pub struct ChatRepository;

impl ChatService for ChatRepository {
    fn get_all_chats(&self, queried_room_id: i32) -> Vec<Chat> {
        use crate::schema::chats::dsl::*;

        let conn = establish_connection();
        chats
            .filter(room_id.eq(queried_room_id))
            .load::<Chat>(&conn)
            .expect("Error loading posts")
    }

    fn create_chat(&self, chat: NewChat) -> Chat {
        let conn = establish_connection();
        diesel::insert_into(chats::table)
            .values(&chat)
            .get_result::<Chat>(&conn)
            .expect("Error occurred during insert a new chat")
    }
}
