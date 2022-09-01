// //! lib.rs
// use actix_web::dev::Server;
// use actix_web::{web, App, HttpResponse, HttpServer};
// use std::net::TcpListener;

// async fn health_check() -> HttpResponse {
//     HttpResponse::Ok().finish()
// }

// // We need to mark `run` as public.
// // Befora, it was no longer a binary entrypoint, therefore we can mark it as async
// // without having to use any proc-macro incantation.
// // We return ``Server` on the happy path and we dropped the `async` keyword.
// // Notice the different signature!
// // We return `Server` on the happy path and we dropped the `async` keyword
// // We have no .await call, so it is not needed anymore.
// pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .listen(listener)?
//         .run();
//     Ok(server)
// }

use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

// greet is an asynchronous function that takes an HttpRequest as input and returns something that implements the Responder trait. A type implements the Responder trait if it can be converted into a HttpResponse - it is implemented off the shelf for a variety of common types (e.g. strings, status codes, bytes, HttpResponse, etc.) and we can roll our own implementations if needed.
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .listen(listener)?
        .run();
    Ok(server)

    // // HttpServer: In charge of all transport level concerns
    // HttpServer::new(|| {
    //     // When the connection is estabished, App handles the requests.
    //     // It follows a builder pattern: new state + new behavior with API by chaining.
    //     App::new().route("/health_check", web::get().to(health_check))
    // })
    // .bind("127.0.0.1:8000")?
    // .run()
    // .await
}
