use diesel::prelude::*;

use crate::domain::model::user::*;
use crate::domain::service::user_service::*;
use crate::schema::users::dsl::*;
use crate::utils::dsl::establish_connection;

pub struct UserRepository;

impl UserService for UserRepository {
    fn get_all_users(&self) -> Vec<User> {
        let connection = establish_connection();
        // TODO: error handling
        users
            .load::<User>(&connection)
            .expect("Error loading posts")
    }
}
