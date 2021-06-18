mod chat_server;
mod repository;
mod utils;
mod schema;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::time::{Instant};

use actix::prelude::*;
use actix_web_actors::ws;
use actix_web::{http, web, get, post, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use crate::chat_server::main_actor::{WsSession};
use crate::chat_server::ws_actor::{WsActor};
use crate::repository::user::get_all_users;
use crate::repository::chat::get_all_chats;
use crate::repository::room::get_all_rooms;
use crate::repository::my_data::{register_my_data, NewMyData};

pub async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<WsActor>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession::new(0, Instant::now(), srv.get_ref().clone()),
        &req,
        stream,
    )
}

#[get("/user/all")]
async fn all_user() -> impl Responder {
    get_all_users()
}

#[get("/chat/all/{room_id}")]
async fn all_chat(room_id: web::Path<i32>) -> impl Responder {
    let ret = get_all_chats(room_id.0);
    println!("{}", ret);
    ret
}

#[get("/room/all")]
async fn all_room() -> impl Responder {
    get_all_rooms()
}

#[post("/mydata/new")]
async fn my_data_home(new_my_data: web::Json<NewMyData>) -> impl Responder {
    register_my_data(new_my_data.into_inner())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ws_server = WsActor::new().start();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "HEAD"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(web::scope("/ws")
                .data(ws_server.clone())
                .service(web::resource("/").to(ws_route)))
            .service(web::scope("rest")
                .service(all_user)
                .service(all_chat)
                .service(my_data_home))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
