use actix_web::{HttpServer, App, middleware::Logger, web};

use config::Config;

mod routes;
mod handlers;
mod models;
mod db;
mod schema;
mod config;
mod auth;
mod errors;

pub struct AppState {
    pub env: Config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pg_connection = db::connection::establish_connection(); // ne pas utiliser en appdate (enfin a vérifier)

    let config = Config::init();

    println!("🚀 Démarrage du back-end.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {env: config.clone()}))
            .wrap(Logger::default())
            .configure(routes::config)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
