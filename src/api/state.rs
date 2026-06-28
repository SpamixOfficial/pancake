use std::{fs, path::PathBuf, sync::Arc};

use color_eyre::eyre::{Result, eyre};
use tracing::{error, info};

use crate::{db::DBClient, config::Config, exists};

// fine grained state to avoid bottlenecks
#[derive(Clone)]
pub struct ApiState {
    pub db: DBClient,
    pub config: Arc<Config>
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

        // needs to be mut for release
        #[allow(unused)]
        let mut db = DBClient::new(data_path, &config).await?;

        db.check_and_apply_pending_migrations(None).await?;

        Ok(ApiState { db, config })
    }
}