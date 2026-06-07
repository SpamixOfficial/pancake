use std::fs;

use color_eyre::eyre::Result;
use axum::{
    Router
};

use crate::frontend::add_frontend_routes;


mod api;
mod app;
mod frontend;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let app = app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    Ok(axum::serve(listener, app?).await?)
}

fn app() -> Result<Router> {
    let dirs = directories::ProjectDirs::from("se.spamix", "", "pancake").unwrap();
    if !exists!(dirs.data_dir()) {
        fs::create_dir(dirs.data_dir())?;
    }

    let mut router = Router::new().nest("/api", api::routes());
    router = add_frontend_routes(&dirs, router)?;
    Ok(router)
}
