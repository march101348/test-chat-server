use actix::prelude::*;
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};

use super::http::routes::*;
use super::web_socket::routes::*;
use crate::application::web_socket::ws_actor::WsActor;

pub async fn run() -> std::io::Result<()> {
    let ws_server = WsActor::new().start();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE);
        App::new()
            .wrap(cors)
            .service(
                web::scope("/ws")
                    .data(ws_server.clone())
                    .service(web::resource("/").to(ws_route)),
            )
            .service(
                web::scope("rest")
                    .service(all_user)
                    .service(all_chat)
                    .service(signup_my_data)
                    .service(signin_my_data),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
