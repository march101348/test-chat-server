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
use actix_web::{web, get, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

use crate::chat_server::main_actor::{WsSession};
use crate::chat_server::ws_actor::{WsActor};
use crate::repository::user::get_all_users;
use crate::repository::chat::{get_all_chats};

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

#[get("/chat/all")]
async fn all_chat() -> impl Responder {
    get_all_chats()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ws_server = WsActor::new().start();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"]);
        App::new()
            .wrap(cors)
            .service(web::scope("/ws")
                .data(ws_server.clone())
                .service(web::resource("/").to(ws_route)))
            .service(web::scope("rest")
                .service(all_user)
                .service(all_chat))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
