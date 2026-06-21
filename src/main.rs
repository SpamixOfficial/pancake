use std::{fs, process::exit};

use axum::{Router, ServiceExt, extract::Request};
use clap::Parser;
use color_eyre::eyre::Result;
use sea_orm::DatabaseConnection;
use tokio::signal;
use tower::Layer;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    api::ApiState, cli::Cli, db::DBClient, frontend::add_frontend_routes, util::get_data_dir,
};

mod api;
mod cli;
mod config;
mod db;
mod frontend;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let cli = Cli::parse();

    info!("Loading .env");
    dotenvy::dotenv()?;

    let (app, db) = app(cli).await?;

    let layer = NormalizePathLayer::trim_trailing_slash().layer(app);

    info!("Starting listener on 0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, ServiceExt::<Request>::into_make_service(layer))
        .with_graceful_shutdown(shutdown())
        .await?;

    info!("Closing database connection");
    db.close().await?;
    info!("Database closed successfully");
    Ok(())
}

async fn app(cli: Cli) -> Result<(Router, DatabaseConnection)> {
    let data_dir = cli.data_directory.clone().unwrap_or_else(|| get_data_dir());
    if !exists!(data_dir) {
        info!(
            "Data directory ({}) does not exist, creating it...",
            data_dir.display()
        );
        fs::create_dir(&data_dir)?;
    }

    info!("Setting up API routes");
    // api state setup
    let mut api_state: ApiState = ApiState::new(&data_dir).await?;
    let db = api_state.db.connection.clone();

    handle_cli(&mut api_state.db, &cli).await?;

    let mut router = Router::new()
        .nest("/api", api::routes(api_state))
        .layer(TraceLayer::new_for_http());

    // Web UI is an optional feature in release
    if cfg!(debug_assertions) || exists!(data_dir.join("web")) {
        info!("Frontend enabled, adding frontend routes");
        router = add_frontend_routes(&data_dir, router)?;
    }
    Ok((router, db))
}

async fn handle_cli(db: &mut DBClient, cli: &Cli) -> Result<()> {
    match &cli.action {
        Some(cli::Command::Migrations { action }) => {
            match action {
                cli::MigrationsCommand::Apply => {
                    db.check_and_apply_pending_migrations(Some(true)).await?;
                }
                cli::MigrationsCommand::RollBack => {
                    db.roll_back_latest_migration().await?;
                }
                cli::MigrationsCommand::Pending => {
                    for m in db.get_pending_migrations().await? {
                        println!("{} - {}", m.name(), m.status())
                    }
                }
                cli::MigrationsCommand::History => {
                    println!("Last updated at {}", db.migration_file.data.last_updated);
                    for m in &db.migration_file.data.migrations {
                        println!(
                            "{} migrations were applied at {}",
                            m.n_migrations, m.applied_at
                        )
                    }
                }
            };
            db.connection.close_by_ref().await?;
            exit(0);
        }
        _ => Ok(()),
    }
}

async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install ctrl_c handler");
    };

    #[cfg(unix)]
    let sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = sigterm => {}
    };

    info!("Shutdown signal received");
}
