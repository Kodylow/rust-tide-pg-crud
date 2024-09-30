use error::AppError;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool};
use tera::Tera;
use tide::http::cookies::SameSite;
use tide::prelude::*;
use tide::{Error, Server};
use tide_tera::prelude::*;
use tracing::info;
use uuid::Uuid;

pub mod config;
pub mod controllers;
pub mod error;
pub mod handlers;

use config::Config;
use controllers::views;
use handlers::server;

use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

static AUTH_URL: &str = "https://accounts.google.com/o/oauth2/auth";
static TOKEN_URL: &str = "https://googleapis.com/oauth2/v3/token";

#[derive(Debug, Clone)]
pub struct AppState {
    db: PgPool,
    tera: Tera,
    oauth_client: BasicClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dinosaur {
    id: Uuid,
    name: String,
    weight: i32,
    diet: String,
    user_id: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), AppError> {
    let config = Config::load()?;

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or("8080".to_string());
    let db_pool = make_db_pool(&config.database_url).await?;
    let oauth_client = make_oauth_client(
        &config.oauth_google_client_id,
        &config.oauth_google_client_secret,
        &config.oauth_google_redirect_url,
    )?;

    let app = server(db_pool).await;
    let mut listener = app.bind(format!("0.0.0.0:{}", port)).await?;

    if let Some(info_log) = listener.info().iter().next() {
        info!("Listening on {}", info_log);
    } else {
        info!("Server started, but unable to retrieve listening information");
    }

    listener.accept().await?;

    Ok(())
}

async fn make_db_pool(db_url: &str) -> Result<PgPool, AppError> {
    let db = Pool::connect(db_url).await?;
    // sqlx::migrate!("./migrations").run(&db).await?;
    Ok(db)
}

fn make_oauth_client(
    client_id: &str,
    client_secret: &str,
    redirect_url: &str,
) -> Result<BasicClient, AppError> {
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        Some(ClientSecret::new(client_secret.to_string())),
        AuthUrl::new(AUTH_URL.to_string()).map_err(|e| AppError::Parse(e))?,
        Some(TokenUrl::new(TOKEN_URL.to_string()).map_err(|e| AppError::Parse(e))?),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).map_err(|e| AppError::Parse(e))?);

    Ok(client)
}
