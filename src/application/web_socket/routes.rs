use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

use super::super::web_socket::main_actor::WsSession;
use super::super::web_socket::ws_actor::WsActor;

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
