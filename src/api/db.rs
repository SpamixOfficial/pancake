use std::{env::Vars, fmt::format, path::PathBuf};

use color_eyre::eyre::{Result, eyre};
use sea_orm::{Database, DatabaseConnection};
use tracing::info;

use crate::config::{
    Config,
    DBType::{self, SQLite},
};

/// Abstraction of ORM queries to make life easier

/// Client for making database queries
#[derive(Debug, Clone)]
pub struct DBClient {
    pub connection: DatabaseConnection,
}

impl DBClient {
    /// Create and connect new client
    pub async fn new(data_path: &PathBuf, conf: &Config) -> Result<Self> {
        info!("Creating database connection");
        let connection_string = create_connection_string(data_path, conf.database.t)?;
        let connection = Database::connect(connection_string).await?;

        Ok(DBClient { connection })
    }
}

fn create_connection_string(data_path: &PathBuf, db_type: DBType) -> Result<String> {
    // ENV var override 
    if let Ok(url) = std::env::var("DB_URL") {
        return Ok(url)
    }
    
    match db_type {
        DBType::MySQL => {
            let user = std::env::var("DB_USERNAME")?;
            let password = std::env::var("DB_PASSWORD")?;
            let host = std::env::var("DB_HOST")?;
            let database = std::env::var("DB_NAME")?;
            Ok(format!("mysql://{user}:{password}@{host}/{database}"))
        }
        SQLite => {
            let db_path = data_path.join("db.sqlite3");
            Ok(format!("sqlite://{}?mode=rwc", db_path.display()))
        }
    }
}
