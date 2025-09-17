use crate::security::oidc::OidcConfig;
use openid::{Bearer, Client, Discovered, StandardClaims};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};

pub struct ActixState {
    pub db_connection: &'static DatabaseConnection,

    pub oidc_config: OidcConfig,
    pub oidc_client: Option<Arc<Mutex<Client<Discovered, StandardClaims>>>>,

    pub internal_store: Mutex<HashMap<String, Bearer>>,
}
