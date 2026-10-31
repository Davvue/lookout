use expanduser::expanduser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    timeout_ms: u32,
    interval_ms: u32,
}

pub fn load_config(path_str: &str) -> Result<Config, std::io::Error> {
    let path = expanduser(path_str)?;

    ensure_directory_structure(&path)?;

    Ok(Config {
        timeout_ms: 1000,
        interval_ms: 1000,
    })
}

fn ensure_directory_structure(path: &Path) -> Result<bool, std::io::Error> {
    if !fs::exists(&path)? {
        fs::create_dir_all(&path)?;
    }

    let config_path = path.join("config.json");
    let services_path = path.join("services.json");
    let outfile_path = path.join("output.json");

    if !fs::exists(&config_path)? {
        let mut f = File::create(&config_path)?;
        let json = serde_json::to_string(&Config {
            timeout_ms: 1000 * 5,
            interval_ms: 1000 * 60 * 10,
        })?;
        f.write(&json.into_bytes())?;
    }

    if !fs::exists(&services_path)? {
        File::create(&services_path)?;
    }

    if !fs::exists(&outfile_path)? {
        File::create(&outfile_path)?;
    }

    Ok(true)
}
