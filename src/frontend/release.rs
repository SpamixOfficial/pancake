use std::{fs, path::PathBuf};

use crate::exists;
use color_eyre::eyre::{Result, eyre};
use tracing::error;
use tower_http::services::ServeDir;

const FRONTEND_BUILD_HASH: &[u8] = include_bytes!("hash");

/*
fn unpack_frontend(web_path: &PathBuf, hash_path: &PathBuf) -> Result<()> {
    let frontend_data = include_bytes!("frontend.7z");
    let data = Cursor::new(frontend_data.as_slice());

    if exists!(web_path) {
        fs::remove_dir_all(&web_path)?;
    }

    sevenz_rust2::decompress(data, web_path)?;

    Ok(fs::write(hash_path, FRONTEND_BUILD_HASH)?)
}
*/
pub fn get_frontend_service(web_path: &PathBuf) -> Result<ServeDir> {
    let hash_path = web_path.join("hash");

    if !exists!(hash_path) || fs::read(&hash_path)? != FRONTEND_BUILD_HASH {
        //unpack_frontend(&web_path, &hash_path)?;
        let _err_msg = "Hash mismatch - please reinstall the frontend";
        error!(_err_msg);
        return Err(eyre!(_err_msg));
    }
    Ok(ServeDir::new(web_path)
        .precompressed_br()
        .precompressed_gzip())
}