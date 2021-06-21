use crate::domain::model::my_data::{MyDataForFront, NewMyData};
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
        let my_data = self.service.create_my_data(new_data);
        let ret = MyDataForFront {
            id: my_data.id,
            name: my_data.name,
        };
        serde_json::to_string(&ret).expect("json parse error in register new my data")
    }
}
