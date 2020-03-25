use actix_cors::Cors;
use actix_web::{http, middleware, Error, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use bytes::{Bytes, BytesMut};
use futures::{StreamExt, TryStreamExt};
use json::JsonValue;
use serde::{Deserialize, Serialize};
use actix_multipart::Multipart;
use std::io::Write;

// use actix_multipart::Multipart;
// use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
// use futures::{StreamExt, TryStreamExt};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            // .wrap(middleware::Logger::default())
            /*
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    // .supports_credentials()
                    // .allowed_origin("http://192.168.147.59:9090/")
                    // .allowed_origin("*")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish()
            )
            */
            // .route("/", web::get().to(index))
            .route("/again", web::get().to(index2))
            // .route("/file", web::post().to(index3))
            // .service(
            //     web::resource("/again")
            //         .route(web::get().to(index))
            //         .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            // )
            /*
            .wrap(
                Cors::new() // <- Construct CORS middleware builder
                    .allowed_origin("http://192.168.147.59:9090/")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish()
            )
            */
            .service(
                web::resource("/file")
                    // .data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(extract_item)),
            )

            /*
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::resource("/file")
                    .route(web::post().to(index3))
                    .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
            )
            */

        // .service(web::resource("/file").route(web::post().to(index3)))
    })
        .bind("192.168.147.59:8088")?
        .run()
        .await
}

async fn index() -> impl Responder {
    println!("index");
    HttpResponse::Ok().body("Hello world!")
}

async fn index2() -> impl Responder {
    println!("index2");

    HttpResponse::Ok().body("Hello world again!")
}

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    description: String,
    // number: i32,
}

async fn index3(item: web::Json<MyObj>) -> impl Responder {
    println!("11111");
    HttpResponse::Ok().body("Hello world again!")
}

/// This handler uses json extractor with limit
/*
async fn extract_item(item: web::Json<MyObj>, req: HttpRequest) -> HttpResponse {
    // println!("request: {:?}", req);
    // println!("model: {:?}", item);

    HttpResponse::Ok().body("Hello world again post!")

    // HttpResponse::Ok().json(item.0) // <- send json response
}
*/

async fn extract_item(mut payload: Multipart) ->  Result<HttpResponse, Error>  {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", filename);
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
