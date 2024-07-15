use actix_web::{middleware::Logger, web, App, HttpServer};

use api_db::connection::Pool;
use api_errors::{ServiceError, ServiceErrorType};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = api_configs::config::Config::init();

    println!("🔒 Authentification par {}.", config.auth_driver);

    let pg_connection: Pool = api_db::connection::establish_connection(&config);

    let redis_client = api_services::redis::get_redis_client(&config);

    let oauth_client =
        api_services::oauth::create_client(&config).expect("Failed to create oauth2 client");

    let users_repository =
        api_db::repositories::users_repository::UsersRepository::new(pg_connection.clone());

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(oauth_client.clone()))
            .wrap(Logger::default())
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
