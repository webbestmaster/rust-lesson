use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod bind;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(bind::hello)
            .service(bind::echo)
            .route("/hey", web::get().to(bind::manual_hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
