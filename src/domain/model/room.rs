use crate::schema::rooms;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Room {
    id: i32,
    name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "rooms"]
pub struct NewRoom<'a> {
    name: &'a str,
}
