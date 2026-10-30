use std::fs;
use std::fs::File;
use std::path::Path;
use expanduser::expanduser;

pub struct Config {}

pub fn load_config(path_str: &str) -> Result<Config, std::io::Error> {
    let path = expanduser(path_str)?;

    ensure_directory_structure(&path)?;

    Ok(Config {})
}

fn ensure_directory_structure(path: &Path) -> Result<bool, std::io::Error> {
    if !fs::exists(&path)? {
        fs::create_dir_all(&path)?;
    }

    let config_path = path.join("config.json");
    let services_path = path.join("services.json");
    let outfile_path = path.join("output.json");

    if !fs::exists(&config_path)? {
        File::create(&config_path)?;
    }

    if !fs::exists(&services_path)? {
        File::create(&services_path)?;
    }

    if !fs::exists(&outfile_path)? {
        File::create(&outfile_path)?;
    }

    Ok(true)
}
