use crc::Crc;
use std::{env::set_current_dir, fs, io::Cursor, process::Command};

fn build_web() {
    println!("cargo::rerun-if-changed=web");
    // Install frontend dependencies
    assert!(set_current_dir("web").is_ok());
    Command::new("bun").args(["install"]).status().unwrap();
    Command::new("bun").args(["run", "build"]).status().unwrap();
    let mut writer = Cursor::new(vec![]);
    let mut archive = sevenz_rust2::ArchiveWriter::new(&mut writer).unwrap();
    archive.push_source_path("build", |_| true).unwrap();
    archive.finish().unwrap();

    let algo = Crc::<u32>::new(&crc::CRC_32_CKSUM);
    let mut algo = algo.digest();
    algo.update(env!("CARGO_PKG_VERSION").as_bytes());
    algo.update(writer.get_ref());
    let hash = algo.finalize();

    assert!(set_current_dir("../").is_ok());

    fs::write("src/frontend/frontend.7z", writer.get_ref()).unwrap();
    fs::write("src/frontend/hash", hash.to_be_bytes()).unwrap();
}

fn add_env_vars() {
    if let Ok(path) = dotenvy::dotenv() {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    
    if let Ok(iter) = dotenvy::dotenv_iter() {
        for i in iter {
            if let Ok((k, v)) = i {
                println!("cargo:rustc-env={}={}", k, v);
            }
        }
    }
}

fn main() {
    #[cfg(not(debug_assertions))]
    build_web();

    add_env_vars();
}
