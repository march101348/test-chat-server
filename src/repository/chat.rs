use diesel::prelude::*;

use crate::utils::dsl::establish_connection;
use crate::schema::chats;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Chat {
    id: i32,
    user_id: i32,
    room_id: i32,
    content: String,
}

pub fn get_all_chats() -> String {
    use crate::schema::chats::dsl::*;

    let conn = establish_connection();
    let results = chats
        .limit(5)
        .load::<Chat>(&conn)
        .expect("Error loading posts");

    serde_json::to_string(&results).unwrap()
}

#[derive(Insertable, Deserialize)]
#[table_name="chats"]
pub struct NewChat<'a> {
    user_id: i32,
    room_id: i32,
    content: &'a str,
}

pub fn create_chat<'a>(chat: NewChat) -> Chat {
    let conn = establish_connection();
    diesel::insert_into(chats::table)
        .values(&chat)
        .get_result::<Chat>(&conn)
        .expect("Error occurred during insert a new chat")
}