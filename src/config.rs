use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Config {
    pub database: DBConfig
}

/// The database configuration struct
/// 
/// The database connection only expects additional fields except for type in case you use a MySQL/MariaDB connection
/// If you use such a connection, additional fields are expected in your ENV variables: DB_PASSWORD, DB_USERNAME, DB_HOST, DB_NAME
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct DBConfig {
    #[serde(rename = "type")]
    pub t: DBType,
}


#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum DBType {
    SQLite,
    MySQL
}