use actix_web::{middleware::Logger, web, App, HttpServer};

use config::Config;
use db::connection::Pool;
use errors::{ServiceError, ServiceErrorType};

mod auth;
mod config;
mod db;
mod errors;
mod extractors;
mod handlers;
mod helpers;
mod models;
mod routes;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_connection: Pool = db::connection::establish_connection(); // ne pas utiliser en appdate (enfin a vérifier)

    let users_repository = db::repositories::users_repository::UsersRepository::new(pg_connection.clone());

    let config: Config = Config::init();

    println!("🚀 Démarrage du back-end.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                ServiceError {
                    message: Some("bad fields".to_string()),
                    error_type: ServiceErrorType::BadDeserialization,
                }
                .into()
            }))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(users_repository.clone()))
            .wrap(Logger::default())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
