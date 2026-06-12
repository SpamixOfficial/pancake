/// Main export layer for api, routes are returned through the routes() method!
///
/// When adding a new api route add it in a **READABLE** way in the routes() method
mod db;

use std::{fs, path::PathBuf, sync::Arc};

use axum::{Router, routing::get};
use color_eyre::eyre::{Result, eyre};
use sea_orm::DatabaseConnection;
use tracing::{error, info};

use crate::{api::db::DBClient, config::Config, exists};

#[derive(Clone)]
pub struct ApiState {
    db: DBClient,
    config: Arc<Config>
}

impl ApiState {
    pub async fn new(data_path: &PathBuf) -> Result<Self> {
        info!("Loading configuration");
        let config_path = data_path.join("config.json");
        if !exists!(config_path) {
            let _err_msg = format!(
                "Config was not found in expected location: {}",
                config_path.display()
            );
            error!(_err_msg);
            return Err(eyre!(_err_msg));
        }
        
        let _data = fs::read(config_path)?;
        let config: Arc<Config> = Arc::new(serde_json::from_slice(&_data)?);

        let db = DBClient::new(data_path, &config).await?;
        Ok(ApiState { db, config })
    }

    pub fn get_db_connection(&self) -> DatabaseConnection {
        self.db.connection.clone()
    }
}

pub fn routes(state: ApiState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .with_state(state)
}
