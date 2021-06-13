mod chat_server;

use std::time::{Instant};
use actix::prelude::*;
use actix_web_actors::ws;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use crate::chat_server::main_actor::{WsSession};
use crate::chat_server::ws_actor::{WsActor};

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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ws_server = WsActor::new().start();
    HttpServer::new(move || {
        App::new()
            .data(ws_server.clone())
            .service(web::resource("/ws/").to(ws_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
