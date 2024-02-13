use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InputUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}
