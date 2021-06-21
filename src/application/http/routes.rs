use actix_web::{
    get, post, web, Responder,
};

use crate::domain::model::my_data::NewMyData;
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
#[get("/room/all")]
async fn all_room() -> impl Responder {
    RoomUsecase::new(RoomRepository).get_all_rooms()
}

#[post("/mydata/new")]
async fn my_data_home(new_my_data: web::Json<NewMyData>) -> impl Responder {
    MyDataUsecase::new(MyDataRepository).register_my_data(new_my_data.into_inner())
}
