use diesel::prelude::*;

use crate::domain::model::my_data::*;
use crate::domain::service::my_data_service::*;
use crate::schema::my_datas;
use crate::utils::dsl::establish_connection;

pub struct MyDataRepository;

impl MyDataService for MyDataRepository {
    fn create_my_data(&self, my_data: NewMyData) -> MyData {
        let conn = establish_connection();
        diesel::insert_into(my_datas::table)
            .values(&my_data)
            .get_result::<MyData>(&conn)
            .expect("Error occurred during insert a new my data")
    }
}
