use serde::{Deserialize, Serialize};

/// Default struct for tokens in JWT authentication.
#[derive(Debug, Serialize, Deserialize)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}
