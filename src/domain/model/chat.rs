use crate::schema::chats;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Chat {
    id: i32,
    user_id: i32,
    room_id: i32,
    content: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "chats"]
pub struct NewChat<'a> {
    user_id: i32,
    room_id: i32,
    content: &'a str,
}
