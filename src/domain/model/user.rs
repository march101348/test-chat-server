#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    age: i32,
}
