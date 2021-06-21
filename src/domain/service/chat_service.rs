use super::super::model::chat::*;

pub trait ChatService {
    fn get_all_chats(&self, queried_room_id: i32) -> Vec<Chat>;
    fn create_chat(&self, chat: NewChat) -> Chat;
}
