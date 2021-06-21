use super::super::model::room_member::*;

pub trait RoomMemberService {
    fn get_all_room_members(queried_room_id: i32) -> Vec<RoomMember>;
    fn create_room_member(room_member: NewRoomMember) -> RoomMember;
}
