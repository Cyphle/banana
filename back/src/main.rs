use crate::config::actix::ActixState;
use crate::config::database::connect;
use crate::config::local::oidc_config::get_oidc_config;
use crate::http::controllers::account::{create_account, find_all, find_one};
use crate::http::controllers::example_actix_session::{delete_session, get_session};
use crate::http::controllers::example_actix_store::{add_to_store, get_from_store};
use crate::http::controllers::profile::get_profile;
use crate::http::controllers::technical::{live, ready};
use crate::http::controllers::test::test;
use crate::security::controllers::login::login;
use crate::security::controllers::logout::logout;
use crate::security::controllers::register::register;
use crate::security::oidc::get_client;
use actix_cors::Cors;
use actix_session::config::PersistentSession;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::{time, Key};
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use openid::{Client, Options, StandardClaims, Token};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod config;
mod domain;
mod dto;
mod http;
mod repositories;
mod security;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::logger::config();

    match config::app_config::AppConfig::new() {
        Ok(config) => {
            // DATABASE
            info!("Connecting to database...");
            let db = connect(&config.database).await.unwrap();
            let static_db = Box::leak(Box::new(db));

            // OIDC
            info!("Configuring OIDC client...");
            let oidc_config = get_oidc_config();
            let oidc_client = Arc::new(Mutex::new(get_client(&oidc_config).await));

            // Session
            info!("Configuring session store...");
            let session_key = Key::from(&[0; 64]);
            let session_store = RedisSessionStore::new(config.get_session_store_url())
                .await
                .unwrap();

            let state = web::Data::new(ActixState {
                db_connection: static_db,

                oidc_config: oidc_config.clone(),
                oidc_client: Some(oidc_client.clone()),

                internal_store: Mutex::new(HashMap::new()),
            });

            info!("Starting Actix server...");
            HttpServer::new(move || {
                App::new()
                    .wrap(
                        Cors::default()
                            .allowed_origin(config.cors.allowed_origin.as_str()) // TODO Change to your frontend URL
                            .allowed_methods(config.cors.allowed_methods.iter().map(|m| m.parse::<reqwest::Method>().unwrap()).collect::<Vec<_>>())
                            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                            .supports_credentials() // Optional, if credentials are used
                            .max_age(3600),
                    )
                    .wrap(
                        SessionMiddleware::builder(session_store.clone(), session_key.clone())
                            .session_lifecycle(
                                PersistentSession::default().session_ttl(time::Duration::days(5)),
                            )
                            .cookie_secure(false)
                            .cookie_name(config.get_cookie_name())
                            .build(),
                    )
                    .app_data(state.clone())
                    // Tests and examples endpoints
                    .service(add_to_store)
                    .service(get_from_store)
                    .service(get_session)
                    .service(delete_session)
                    // End tests and examples
                    .service(login)
                    .service(logout)
                    .service(register)
                    .service(get_profile)
                    .service(create_account)
                    .service(find_one)
                    .service(find_all)
                    .service(test)
                    // Technical
                    .service(live)
                    .service(ready)
            })
                .bind("0.0.0.0:8080")?
                .run()
                .await
        }
        Err(e) => {
            log::error!("Error while loading application configuration: {:?}.\nCannot start server.", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Configuration error"));
        }
    }
    
    // TODO faudra trouver un moyen de close la connexion. Mais là on peut pas move la static_db
    // TODO en fait il faut plutôt clone plutôt qu'un pointeur. et même sans ça, la connexion se close bien
}
