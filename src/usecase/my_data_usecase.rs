use crate::domain::model::my_data::{NewMyData, SignInData};
use crate::domain::service::my_data_service::MyDataService;

pub struct MyDataUsecase<Service: MyDataService> {
    service: Service,
}

impl<Service> MyDataUsecase<Service>
where
    Service: MyDataService,
{
    pub fn new(service: Service) -> Self {
        MyDataUsecase { service }
    }

    pub fn register_my_data(&self, new_data: NewMyData) -> String {
        let my_data = self
            .service
            .create_my_data(new_data)
            .into_my_data_for_front();
        serde_json::to_string(&my_data).expect("json parse error in register new my data")
    }

    pub fn get_my_data(&self, sign_in_data: SignInData) -> String {
        let my_data = self
            .service
            .get_my_data(sign_in_data)
            .into_my_data_for_front();
        serde_json::to_string(&my_data).expect("json parse error in get my data")
    }
}
