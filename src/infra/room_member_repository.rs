use diesel::prelude::*;

use crate::domain::model::room_member::*;
use crate::domain::service::room_member_service::*;
use crate::schema::room_members;
use crate::utils::dsl::establish_connection;

struct RoomMemberRepository;

impl RoomMemberService for RoomMemberRepository {
    fn get_all_room_members(queried_room_id: i32) -> Vec<RoomMember> {
        use crate::schema::room_members::dsl::*;

        let conn = establish_connection();
        // TODO: error handling
        room_members
            .filter(room_id.eq(queried_room_id))
            .load::<RoomMember>(&conn)
            .expect("Error loading posts")
    }

    fn create_room_member(room_member: NewRoomMember) -> RoomMember {
        let conn = establish_connection();
        // TODO: error handling
        diesel::insert_into(room_members::table)
            .values(&room_member)
            .get_result::<RoomMember>(&conn)
            .expect("Error occurred during insert a new room member")
    }
}
