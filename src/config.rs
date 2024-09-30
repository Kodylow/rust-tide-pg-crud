use serde::Deserialize;

use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub port: String,
    pub oauth_google_client_id: String,
    pub oauth_google_client_secret: String,
    pub oauth_google_redirect_url: String,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let port = std::env::var("PORT").expect("PORT must be set");
        let oauth_google_client_id =
            std::env::var("OAUTH_GOOGLE_CLIENT_ID").expect("OAUTH_GOOGLE_CLIENT_ID must be set");
        let oauth_google_client_secret = std::env::var("OAUTH_GOOGLE_CLIENT_SECRET")
            .expect("OAUTH_GOOGLE_CLIENT_SECRET must be set");
        let oauth_google_redirect_url = std::env::var("OAUTH_GOOGLE_REDIRECT_URL")
            .expect("OAUTH_GOOGLE_REDIRECT_URL must be set");

        Ok(Config {
            database_url,
            port,
            oauth_google_client_id,
            oauth_google_client_secret,
            oauth_google_redirect_url,
        })
    }
}
