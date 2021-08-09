use actix_web::{get, post, web, Responder};

use crate::domain::model::my_data::NewMyData;
use crate::domain::model::my_data::SignInData;
use crate::domain::model::room::NewRoomForFront;
use crate::infra::chat_repository::ChatRepository;
use crate::infra::my_data_repository::MyDataRepository;
use crate::infra::room_repository::RoomRepository;
use crate::infra::user_repository::UserRepository;
use crate::usecase::chat_usecase::ChatUsecase;
use crate::usecase::my_data_usecase::MyDataUsecase;
use crate::usecase::room_usecase::RoomUsecase;
use crate::usecase::user_usecase::UserUsecase;

// TODO: repositoryを外部から設定するよう変更

// TODO: 件数指定で取得に変更（無限スクロール用）
#[get("/user/all")]
async fn all_user() -> impl Responder {
    UserUsecase::new(UserRepository).get_all_users()
}

// TODO: 件数指定で取得に変更（無限スクロール用）
#[get("/chat/all/{room_id}")]
async fn all_chat(room_id: web::Path<i32>) -> impl Responder {
    ChatUsecase::new(ChatRepository).get_all_chats(room_id.0)
}

// TODO: 件数指定で取得に変更（無限スクロール用）
#[get("/room/all/{user_id}")]
async fn all_room(user_id: web::Path<i32>) -> impl Responder {
    RoomUsecase::new(RoomRepository).get_all_rooms(user_id.0)
}

#[post("/room/create")]
async fn create_room(new_room: web::Json<NewRoomForFront>) -> impl Responder {
    RoomUsecase::new(RoomRepository).create_room(new_room.into_inner())
}

#[post("/mydata/signup")]
async fn signup_my_data(new_my_data: web::Json<NewMyData>) -> impl Responder {
    MyDataUsecase::new(MyDataRepository).register_my_data(new_my_data.into_inner())
}

#[post("/mydata/signin")]
async fn signin_my_data(sign_in_data: web::Json<SignInData>) -> impl Responder {
    MyDataUsecase::new(MyDataRepository).get_my_data(sign_in_data.into_inner())
}
