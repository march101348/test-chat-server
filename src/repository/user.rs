use diesel::prelude::*;

use crate::utils::dsl::establish_connection;

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    age: i32,
}

pub fn get_all_users() -> String {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for user in &results {
        println!("{}", user.name);
        println!("----------\n");
        println!("{}", user.age);
    }
    serde_json::to_string(&results).unwrap()
}