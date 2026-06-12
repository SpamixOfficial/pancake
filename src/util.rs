use std::{env::{Vars, VarsOs}, path::PathBuf};

#[macro_export]
macro_rules! exists {
    ($x:expr) => {
        $x.try_exists().map_or(false, |x| x)
    };
}

pub fn get_data_dir() -> PathBuf {
    if let Ok(x) = std::env::var("PANCAKE_DATA_DIR") {
        return PathBuf::from(x);
    }

    #[cfg(target_family = "windows")]
    let path = {
        let base = std::env::var("ProgramData").unwrap_or_else(|_| "C:\\ProgramData".to_string());
        PathBuf::from(base).join("pancake")
    };
    #[cfg(target_family = "unix")]
    let path = PathBuf::from("/usr/share/pancake");

    return path;
}