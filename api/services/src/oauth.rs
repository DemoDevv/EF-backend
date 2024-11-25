use api_configs::config::Config;

use api_errors::ServiceError;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use reqwest::Client;
use serde::Deserialize;

pub fn create_client(config: &Config) -> Result<BasicClient, oauth2::url::ParseError> {
    Ok(BasicClient::new(
        ClientId::new(config.oauth_info.oauth_client_id.clone()),
        Some(ClientSecret::new(
            config.oauth_info.oauth_client_secret.clone(),
        )),
        AuthUrl::new(config.oauth_info.oauth_auth_url.clone())?,
        Some(TokenUrl::new(config.oauth_info.oauth_token_url.clone())?),
    )
    .set_redirect_uri(RedirectUrl::new(
        config.oauth_info.oauth_redirect_url.clone(),
    )?))
}

#[derive(Debug, Deserialize)]
pub struct GoogleUserInfo {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
}

pub async fn fetch_google_user_info(access_token: &str) -> Result<GoogleUserInfo, ServiceError> {
    let client = Client::new();
    let response: GoogleUserInfo = client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(|e| ServiceError::from(e))?
        .json()
        .await
        .map_err(|e| ServiceError::from(e))?;

    Ok(response)
}
