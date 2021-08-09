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

    fn get_my_data(&self, sign_in_data: SignInData) -> MyData {
        use crate::schema::my_datas::dsl::*;

        let conn = establish_connection();

        // TODO: error handling
        let mut results: Vec<MyData> = my_datas
            .filter(my_id.eq(sign_in_data.my_id))
            .filter(password.eq(sign_in_data.password))
            .load::<MyData>(&conn)
            .expect("Error loading posts");
        if results.len() != 1 {
            panic!("no match data for given sign in data");
        }
        results.pop().unwrap()
    }
}
