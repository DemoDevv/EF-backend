use actix_web::web;

use api_db::repositories::users_repository::UsersRepository;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(api_handlers::health::service)
            .configure(api_handlers::auth::service::<UsersRepository>)
            .configure(api_handlers::secure::service)
            .configure(api_handlers::users::service),
    );
}
