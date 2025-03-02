use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use api_db::connection::Pool;
use api_errors::{ServiceError, ServiceErrorType};
use api_middlewares::rate_limiter::RateLimiter;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = api_configs::config::Config::init();

    println!("⚙️ Initialisation des bases de données.");
    let pg_connection: Pool = api_db::connection::establish_connection(&config);
    let redis_client = api_caches::redis::get_redis_client(&config);

    println!("⚙️ Initialisation du client oauth.");
    let oauth_client =
        api_services::oauth::create_client(&config).expect("Failed to create oauth2 client");

    // instanciation des caches
    println!("⚙️ Instanciation des caches.");
    let access_refresh_tokens_cache = Arc::new(
        api_caches::access_refresh_tokens::AccessRefreshTokensCacheRedis::new(
            Arc::clone(&redis_client),
            config.clone(),
        ),
    );

    let rate_limiter_cache = Arc::new(api_caches::token_buckets::TokenBucketsCacheRedis::new(
        Arc::clone(&redis_client),
    ));

    println!("⚙️ Création des répositories pour injection de dépendances.");
    let users_repository = Arc::new(
        api_db::repositories::users_repository::UsersRepository::new(Arc::clone(&pg_connection)),
    );

    // instanciation des services
    println!("⚙️ Instanciation des services.");
    let user_service = api_services::users::UsersService::new(Arc::clone(&pg_connection));
    let auth_service = api_services::auth::services::AuthService::new(
        Arc::clone(&users_repository),
        access_refresh_tokens_cache,
    );

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("🚀 Démarrage du back-end.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .app_data(web::JsonConfig::default().error_handler(|_, _| {
                ServiceError {
                    message: Some("bad fields".to_string()),
                    error_type: ServiceErrorType::BadDeserialization,
                }
                .into()
            }))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .app_data(web::Data::new(redis_client.clone()))
            .app_data(web::Data::new(oauth_client.clone()))
            .app_data(web::Data::new(rate_limiter_cache.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(RateLimiter)
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
