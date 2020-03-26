use actix_web::{HttpServer, App};
use actix_files as fs;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./www").show_files_listing())
            .service(fs::Files::new("/favicon.ico", "./www").index_file("favicon.ico"))
            .service(fs::Files::new("/robots.txt", "./www").index_file("robots.txt"))
            .service(fs::Files::new("/", "./www").index_file("index.html"))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
