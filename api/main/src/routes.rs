use actix_web::web;

use api_lib::repositories::users_repository::UsersRepository;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(api_lib::handlers::health::service)
            .configure(api_lib::handlers::auth::service::<UsersRepository>)
            .configure(api_lib::handlers::secure::service),
    );
}
