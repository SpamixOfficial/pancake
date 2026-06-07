use std::{io::Cursor, path::PathBuf, fs};

use color_eyre::eyre::Result;
use directories::ProjectDirs;
use tower_http::services::ServeDir;
use crate::exists;


const FRONTEND_BUILD_HASH: &[u8] = include_bytes!("hash");

fn unpack_frontend(web_path: &PathBuf, hash_path: &PathBuf) -> Result<()> {
    let frontend_data = include_bytes!("frontend.7z");
    let data = Cursor::new(frontend_data.as_slice());

    if exists!(web_path) {
        fs::remove_dir_all(&web_path)?;
    }

    sevenz_rust2::decompress(data, web_path)?;

    Ok(fs::write(hash_path, FRONTEND_BUILD_HASH)?)
}

pub fn get_frontend_service(dirs: &ProjectDirs) -> Result<ServeDir> {
    let web_path = dirs.data_dir().join("web");
    let hash_path = web_path.join("hash");

    if !exists!(web_path) || !exists!(hash_path) || fs::read(&hash_path)? != FRONTEND_BUILD_HASH {
        unpack_frontend(&web_path, &hash_path)?;
    }
    Ok(ServeDir::new(web_path)
        .precompressed_br()
        .precompressed_gzip())
}