use api_configs::config::Config;

use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use oauth2::basic::BasicClient;

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
