use diesel::prelude::*;

use crate::domain::model::room::{NewRoom, NewRoomForFront, Room};
use crate::domain::model::room_member::NewRoomMember;
use crate::domain::service::room_service::RoomService;
use crate::schema::room_members;
use crate::schema::rooms;
use crate::utils::dsl::establish_connection;

pub struct RoomRepository;

impl RoomService for RoomRepository {
    fn get_all_rooms(&self, queried_id: i32) -> Vec<Room> {
        let conn = establish_connection();
        rooms::table
            .inner_join(room_members::table.on(rooms::id.eq(room_members::room_id)))
            .filter(room_members::user_id.eq(queried_id))
            .select((rooms::id, rooms::name))
            .load::<Room>(&conn)
            .expect("Error loading all rooms")
    }

    fn create_room(&self, new_room_front: NewRoomForFront) -> Room {
        let new_room = NewRoom {
            name: new_room_front.name,
        };
        let conn = establish_connection();
        let room = diesel::insert_into(rooms::table)
            .values(&new_room)
            .get_result::<Room>(&conn)
            .expect("Error occurred during insert a new room");
        new_room_front.member.into_iter().for_each(|user_id| {
            diesel::insert_into(room_members::table)
                .values(&NewRoomMember {
                    user_id,
                    room_id: room.id,
                })
                .execute(&conn)
                .expect("Error occured during insert a new room member");
        });
        room
    }
}
