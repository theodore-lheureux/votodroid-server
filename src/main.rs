use std::env;

use actix_cors::Cors;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::Key,
    middleware,
    web::{self, Data},
    App, HttpServer,
};
#[cfg(debug_assertions)]
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use votodroid_server::{graphql_route, schema};

#[cfg(not(debug_assertions))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let secret_key = Key::generate();
    let redis_url = "127.0.0.1:6379";

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(Cors::permissive())
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::builder(redis_url).build(),
                secret_key.clone(),
            ))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
    });

    server.bind("127.0.0.1:8080").unwrap().run().await
}

#[cfg(debug_assertions)]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let secret_key = Key::generate();
    let redis_url = "127.0.0.1:6379";

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("nopass.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(Cors::permissive())
            .wrap(SessionMiddleware::new(
                RedisActorSessionStore::builder(redis_url).build(),
                secret_key.clone(),
            ))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
    });
    server
        .bind_openssl("127.0.0.1:8080", builder)
        .unwrap()
        .run()
        .await
}
