use super::super::model::my_data::*;

pub trait MyDataService {
    fn create_my_data(&self, new_data: NewMyData) -> MyData;
    fn get_my_data(&self, sign_in_data: SignInData) -> MyData;
}
