use crate::schema::rooms;

// pub付けて良いものか？
#[derive(Queryable, Serialize, Deserialize)]
pub struct Room {
    pub id: i32,
    name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "rooms"]
pub struct NewRoom {
    pub name: String,
}

#[derive(Deserialize)]
pub struct NewRoomForFront {
    pub name: String,
    pub member: Vec<i32>,
}
