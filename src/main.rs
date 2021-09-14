use actix_web::HttpServer;
#[actix_web::main]
#[actix_web::main]
async fn main() -> Result<(), Error> {

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(handlers::encrypt::execute)
            .service(handlers::decrypt::execute)
    })
    .workers(workers)
    .bind(addr)?
    .run()
    .await
}