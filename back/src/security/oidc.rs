use openid::{Client, StandardClaims};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct OidcAdminConfig {
    pub client: OidcClientConfig,
    pub create_user_url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OidcClientConfig {
    pub id: String,
    pub secret: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct OidcConfig {
    pub url: String,
    pub realm_name: String,
    pub redirect_uri: String,
    pub logout_uri: String,
    pub client: OidcClientConfig,
    pub nonce: String,
    pub session_timeout_minutes: i64,
    pub admin: OidcAdminConfig,
}

pub async fn get_client(config: &OidcConfig) -> Client<openid::Discovered, StandardClaims> {
    Client::discover(
        config.client.id.to_string(),
        config.client.secret.to_string(),
        Some(config.redirect_uri.to_string()),
        reqwest::Url::parse(&config.url).unwrap(),
    )
    .await
    .expect("Failed to discover OpenID configuration")
}
