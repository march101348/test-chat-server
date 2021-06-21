use crate::domain::service::user_service::UserService;

pub struct UserUsecase<Service: UserService> {
    service: Service,
}

impl<Service> UserUsecase<Service>
where
    Service: UserService,
{
    pub fn new(service: Service) -> Self {
        UserUsecase { service }
    }

    pub fn get_all_users(&self) -> String {
        let users = self.service.get_all_users();
        serde_json::to_string(&users).expect("json parse error in get all users")
    }
}
