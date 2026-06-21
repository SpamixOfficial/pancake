use std::{fs, path::PathBuf};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::{DateTime, Utc};
use color_eyre::eyre::{Result, eyre};
use sea_orm::{Database, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::{
    config::{
        Config,
        DBType::{self, SQLite},
    },
    exists,
};

pub mod entity;

use migration::{Migration, Migrator, MigratorTrait};

/// Abstraction of ORM queries to make life easier

/// Client for making database queries
#[derive(Debug, Clone)]
pub struct DBClient {
    pub connection: DatabaseConnection,
    pub auto_apply_migrations: bool,
    pub migration_file: DBMigrationFile,
}

/**
    ---------------------- Boilerplate and utility ----------------------
*/
impl DBClient {
    /// Create and connect new client
    pub async fn new(data_path: &PathBuf, conf: &Config) -> Result<Self> {
        info!("Creating database connection");
        let connection_string = create_connection_string(data_path, conf.database.t)?;
        let connection = Database::connect(connection_string).await?;

        let auto_apply_migrations = conf.database.auto_apply_migrations;

        let migration_file = load_migration_file(data_path)?;

        Ok(DBClient {
            connection,
            auto_apply_migrations,
            migration_file,
        })
    }

    pub async fn check_and_apply_pending_migrations(&mut self, apply: Option<bool>) -> Result<()> {
        let n_migrations = Migrator::get_pending_migrations(&self.connection)
            .await?
            .len() as u32;

        if n_migrations == 0 {
            return Ok(());
        }

        if apply == Some(false) {
            warn!("Database has pending migrations but apply is false");
            return Ok(());
        }

        if self.auto_apply_migrations || apply.unwrap_or(false) {
            info!("Applying {} migrations", n_migrations);
            self.apply_pending_migrations(n_migrations).await?;
        } else {
            warn!(
                "Database has pending migrations but auto_apply_migrations is false, please apply migrations either through the CLI or web UI"
            );
        }
        Ok(())
    }

    async fn apply_pending_migrations(&mut self, n_migrations: u32) -> Result<()> {
        info!("Applying {} migrations", n_migrations);
        Migrator::up(&self.connection, None).await?;

        let _date = Utc::now();
        let migration = DBMigration {
            n_migrations,
            applied_at: _date,
        };
        self.migration_file.data.migrations.push(migration);
        self.migration_file.data.last_updated = _date;
        fs::write(
            &self.migration_file.path,
            serde_json::to_string_pretty(&self.migration_file.data)?,
        )?;
        Ok(())
    }

    pub async fn roll_back_latest_migration(&mut self) -> Result<()> {
        let latest_migration = self
            .migration_file
            .data
            .migrations
            .pop()
            .ok_or(eyre!("Database has no migrations to roll back"))?;

        info!("Rolling back {} migrations", latest_migration.n_migrations);
        Migrator::down(&self.connection, Some(latest_migration.n_migrations)).await?;

        Ok(())
    }

    pub async fn get_pending_migrations(&self) -> Result<Vec<Migration>> {
        Ok(Migrator::get_pending_migrations(&self.connection).await?)
    }
}

/*
    ---------------------- Client methods ----------------------
*/
impl DBClient {
    pub async fn authenticate_user_by_email(
        &self,
        email: String,
        password: String,
    ) -> Result<entity::user::Model> {
        let user = self.get_user_by_email(email).await?;
        let hash = PasswordHash::new(&user.password_hash)?;
        if Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
        {
            Ok(user)
        } else {
            Err(eyre!("Invalid password"))
        }
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<entity::user::Model> {
        entity::user::Entity::find_by_email(email)
            .one(&self.connection)
            .await?
            .ok_or(eyre!("No such user"))
    }
}

fn create_connection_string(data_path: &PathBuf, db_type: DBType) -> Result<String> {
    // ENV var override
    if let Ok(url) = std::env::var("DB_URL") {
        return Ok(url);
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

#[derive(Debug, Clone)]
pub struct DBMigrationFile {
    pub data: DBMigrationData,
    path: PathBuf,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct DBMigrationData {
    pub migrations: Vec<DBMigration>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DBMigration {
    pub n_migrations: u32,
    pub applied_at: DateTime<Utc>,
}

fn load_migration_file(data_path: &PathBuf) -> Result<DBMigrationFile> {
    let path: PathBuf = data_path.join("migration.json");
    let data: DBMigrationData;
    if exists!(path) {
        let _data = fs::read(&path)?;
        data = serde_json::from_slice(&_data)?;
    } else {
        data = DBMigrationData::default();
    }

    Ok(DBMigrationFile { data, path })
}
