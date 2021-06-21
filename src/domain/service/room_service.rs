use super::super::model::room::*;

pub trait RoomService {
    fn get_all_rooms(&self) -> Vec<Room>;
    fn create_room<'a>(&self, room: NewRoom) -> Room;
}
