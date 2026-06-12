use std::path::PathBuf;

use axum::Router;
use color_eyre::eyre::Result;

#[cfg(debug_assertions)]
mod development;
#[cfg(not(debug_assertions))]
mod release;

pub fn add_frontend_routes(data_dir: &PathBuf, mut router: Router) -> Result<Router> {
    #[cfg(not(debug_assertions))]
    {
        use crate::frontend::release::get_frontend_service;
        let _frontend_service = get_frontend_service(data_dir)?;
        router = router.fallback_service(_frontend_service)
    }
    #[cfg(debug_assertions)]
    {
        use axum::routing::any;
        use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};

        use crate::frontend::development::{Client, get_frontend_service};

        let _client: Client =
            hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
                .build(HttpConnector::new());
        router = router.fallback(any(get_frontend_service).with_state(_client));
    }
    Ok(router)
}
