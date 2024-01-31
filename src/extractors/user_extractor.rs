use serde::Deserialize;

#[derive(Deserialize)]
pub struct InputUser {
    pub email: String,
    pub password: String,
}
