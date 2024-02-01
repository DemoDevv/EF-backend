use actix_web::web;

use crate::db::repositories::users_repository::UsersRepository;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(crate::handlers::health_handlers::service)
            .configure(crate::handlers::auth_handlers::service::<UsersRepository>)
            .configure(crate::handlers::secure_handlers::service),
    );
}
