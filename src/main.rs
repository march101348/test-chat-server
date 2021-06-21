mod application;
mod domain;
mod infra;
mod schema;
mod usecase;
mod utils;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::application::server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server::init().await
}
