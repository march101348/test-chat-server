use crate::domain::service::room_service::RoomService;
use crate::domain::model::room::NewRoomForFront;

pub struct RoomUsecase<Service: RoomService> {
    service: Service,
}

impl<Service> RoomUsecase<Service>
where
    Service: RoomService,
{
    pub fn new(service: Service) -> Self {
        RoomUsecase { service }
    }

    pub fn get_all_rooms(&self, user_id: i32) -> String {
        let rooms = self.service.get_all_rooms(user_id);
        serde_json::to_string(&rooms).expect("json parse error in get all rooms")
    }

    pub fn create_room(&self, new_room: NewRoomForFront) -> String {
        let room = self.service.create_room(new_room);
        serde_json::to_string(&room).expect("json parse error in create new room")
    }
}
