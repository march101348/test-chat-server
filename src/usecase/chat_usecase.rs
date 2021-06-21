use crate::domain::model::chat::NewChat;
use crate::domain::service::chat_service::ChatService;

pub struct ChatUsecase<Service: ChatService> {
    service: Service,
}

impl<Service> ChatUsecase<Service>
where
    Service: ChatService,
{
    pub fn new(service: Service) -> Self {
        ChatUsecase { service }
    }

    pub fn get_all_chats(&self, room_id: i32) -> String {
        let chats = self.service.get_all_chats(room_id);
        serde_json::to_string(&chats).expect("json parse error in get all chats")
    }

    pub fn register_new_chat(&self, chat: NewChat) -> String {
        let chat = self.service.create_chat(chat);
        serde_json::to_string(&chat).expect("json parse error in register new chat")
    }
}
