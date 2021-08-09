use super::super::model::room::*;

pub trait RoomService {
    fn get_all_rooms(&self, user_id: i32) -> Vec<Room>;
    fn create_room(&self, room: NewRoomForFront) -> Room;
}
