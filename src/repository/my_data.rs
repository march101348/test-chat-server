use diesel::prelude::*;

use crate::utils::dsl::establish_connection;
use crate::schema::my_datas;

#[derive(Serialize, Deserialize)]
pub struct MyDataForFront {
    id: i32,
    name: String,
}

pub fn register_my_data(new_data: NewMyData) -> String {
    let my_data = create_my_data(new_data);
    let ret = MyDataForFront{ id: my_data.id, name: my_data.name };
    serde_json::to_string(&ret).unwrap()
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct MyData {
    id: i32,
    my_id: String,
    password: String,
    name: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="my_datas"]
pub struct NewMyData {
    my_id: String,
    password: String,
    name: String,
}

pub fn create_my_data(my_data: NewMyData) -> MyData {
    let conn = establish_connection();
    diesel::insert_into(my_datas::table)
        .values(&my_data)
        .get_result::<MyData>(&conn)
        .expect("Error occurred during insert a new my data")
}