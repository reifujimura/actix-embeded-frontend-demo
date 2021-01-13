mod frontend;

use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::scope("/").default_service(web::get().to(frontend::get)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
