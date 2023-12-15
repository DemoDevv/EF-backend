use actix_web::{HttpServer, App, middleware::Logger};

mod routes;
mod handlers;
mod models;
mod db;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pg_connection = db::connection::establish_connection(); // ne pas utiliser en appdate (enfin a vérifier)

    println!("[INFO]: Démarrage du back-end.");
    println!("[INFO]: Connexion à la base de données réussite.");
    println!("[INFO]: Le back-end écoute sur le port 8080 à l'adresse 127.0.0.1.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(routes::config)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
