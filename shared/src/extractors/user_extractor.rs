use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct InputUser {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdatableUser {
    #[validate(email)]
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct RefreshableUser {
    #[validate(email)]
    pub email: String,
    pub refresh_token: String,
}
