use crate::domain::service::room_service::RoomService;

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

    pub fn get_all_rooms(&self) -> String {
        let rooms = self.service.get_all_rooms();
        serde_json::to_string(&rooms).expect("json parse error in get all rooms")
    }
}
