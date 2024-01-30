use actix_web::{Error, HttpResponse, web};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::auth::services::decode_token;
use crate::config::Config;

pub async fn test(config: web::Data<Config>, token: BearerAuth) -> Result<HttpResponse, Error> {
    // j'unwrap car je sais que le token est valide étant donné que j'ai passé le middleware
    let claims = decode_token(config, token.token()).unwrap().claims;

    Ok(HttpResponse::Ok().json("good token, you have the role: ".to_string() + &claims.role))
}