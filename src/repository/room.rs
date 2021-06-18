use diesel::prelude::*;

use crate::utils::dsl::establish_connection;
use crate::schema::rooms;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Room {
    id: i32,
    name: String,
}

pub fn get_all_rooms() -> String {
    use crate::schema::rooms::dsl::*;

    let conn = establish_connection();
    let results = rooms
        .limit(5)
        .load::<Room>(&conn)
        .expect("Error loading posts");

    serde_json::to_string(&results).unwrap()
}

#[derive(Insertable, Deserialize)]
#[table_name="rooms"]
pub struct NewRoom<'a> {
    name: &'a str,
}

pub fn create_chat<'a>(room: NewRoom) -> Room {
    let conn = establish_connection();
    diesel::insert_into(rooms::table)
        .values(&room)
        .get_result::<Room>(&conn)
        .expect("Error occurred during insert a new room")
}