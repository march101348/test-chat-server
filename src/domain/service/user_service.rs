use super::super::model::user::User;

pub trait UserService {
    fn get_all_users(&self) -> Vec<User>;
}
