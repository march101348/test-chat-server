use crate::schema::room_members;

#[derive(Queryable, Serialize, Deserialize)]
pub struct RoomMember {
    user_id: i32,
    room_id: i32,
}

#[derive(Insertable, Deserialize)]
#[table_name = "room_members"]
pub struct NewRoomMember {
    user_id: i32,
    room_id: i32,
}
