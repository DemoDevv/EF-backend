use actix_web::web;

use api_caches::access_refresh_tokens::AccessRefreshTokensCacheRedis;
use api_db::repositories::users_repository::UsersRepository;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(api_handlers::health::service)
            .configure(
                api_handlers::auth::service::<UsersRepository, AccessRefreshTokensCacheRedis>,
            )
            .configure(api_handlers::secure::service)
            .configure(api_handlers::users::service::<UsersRepository>)
            .configure(
                api_handlers::oauth::service::<UsersRepository, AccessRefreshTokensCacheRedis>,
            ),
    );
}
