use std::io::Write;
use actix_web::{web, HttpServer, App, Error, HttpResponse};
use actix_files::{Files};
use futures::{StreamExt, TryStreamExt};
use actix_multipart::Multipart;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/post-file")
                    // .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(save_file)),
            )
            .service(Files::new("/static", "./www").show_files_listing())
            .service(Files::new("/favicon.ico", "./www").index_file("favicon.ico"))
            .service(Files::new("/robots.txt", "./www").index_file("robots.txt"))
            .service(Files::new("/", "./www").index_file("index.html"))
    })
        .bind("localhost:8080")?
        .run()
        .await
}

async fn save_file(mut payload: Multipart) ->  Result<HttpResponse, Error>  {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./saved-files/{}", filename);
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

