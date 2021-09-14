use actix_web::{ 
    middleware::{Compress, Condition, Logger},
    App, HttpServer, get, HttpResponse, Responder
};

use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};
use std::{io::Error, result::Result};

mod config;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> Result<(), Error> {
    let addr = config::get_server_address();
    let workers = config::get_worker_count();

    //let enable_auth = std::env::var("ENABLE_AUTH") == Ok("true".into());

    info!("Server Address: {}", &addr);
    info!("Worker threads: {}", &workers);

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(hello)
    })
    .workers(2)
    .bind(addr)?
    .run()
    .await
}