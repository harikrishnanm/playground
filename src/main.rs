use actix_web::{
    get,
    middleware::{Compress, Condition, Logger},
    web, App, HttpResponse, HttpServer, Responder,
};

use dotenv::dotenv;
use env_logger::Env;
use log::{debug, info};
use std::collections::HashMap;
use std::thread;
use std::{
    io::Error,
    sync::{Arc, Mutex},
};

mod config;
mod middleware;

use middleware::Authenticate;

#[get("/read")]
async fn hello(data: web::Data<AppState>) -> impl Responder {
    info!("Counter {:?}", thread::current().id());
    HttpResponse::Ok().body("Hello world!")
}

#[get("/add")]
async fn clear(data: web::Data<AppState>) -> impl Responder {
    let mut map = data.counter.lock().unwrap();
    map.insert(String::from("Hello1"), String::from("workld1"));
    HttpResponse::Ok().body("Added")
}

#[derive(Debug)]
pub struct AppState {
    counter: Mutex<HashMap<String, String>>,
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    let addr = config::get_server_address();
    let workers = config::get_worker_count();

    //let enable_auth = std::env::var("ENABLE_AUTH") == Ok("true".into());

    info!("Server Address: {}", &addr);
    info!("Worker threads: {}", &workers);
    let mut map = HashMap::new();
    map.insert(String::from("Hello"), String::from("World"));
    let app_state = web::Data::new(AppState {
        counter: Mutex::new(map),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(Condition::new(true, Authenticate))
            .service(hello)
            .service(clear)
    })
    .workers(2)
    .bind(addr)?
    .run()
    .await
}
