use diesel::prelude::*;

use crate::domain::model::room::*;
use crate::domain::service::room_service::*;
use crate::schema::rooms;
use crate::utils::dsl::establish_connection;

pub struct RoomRepository;

impl RoomService for RoomRepository {
    fn get_all_rooms(&self) -> Vec<Room> {
        use crate::schema::rooms::dsl::*;

        let conn = establish_connection();
        rooms.load::<Room>(&conn).expect("Error loading posts")
    }

    fn create_room<'a>(&self, room: NewRoom) -> Room {
        let conn = establish_connection();
        diesel::insert_into(rooms::table)
            .values(&room)
            .get_result::<Room>(&conn)
            .expect("Error occurred during insert a new room")
    }
}
