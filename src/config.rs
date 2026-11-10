use expanduser::expanduser;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    id: String,
    name: String,
    url: String,
    timeout_ms: Option<u32>,
    interval_ms: Option<u32>,
}

impl Service {
    pub fn id(&self) -> &str { &self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn url(&self) -> &str { &self.url }
    pub fn timeout_ms(&self) -> Option<u32> { self.timeout_ms }
    pub fn interval_ms(&self) -> Option<u32> { self.interval_ms }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunResult {
    id: String,
    failed_attempts: u8,
}

impl RunResult {
    pub fn new(id: String, failed_attempts: u8) -> Self {
        Self { id, failed_attempts }
    }
    
    pub fn id(&self) -> &str { &self.id }
    pub fn failed_attempts(&self) -> u8 { self.failed_attempts }
    pub fn increment_attempts(&mut self) {
        self.failed_attempts = self.failed_attempts.saturating_add(1);
    }
    pub fn reset_attempts(&mut self) {
        self.failed_attempts = 0;
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OutputList {
    last_run_ms: Option<u64>,
    outputs: Vec<RunResult>,
}

impl OutputList {
    pub fn new(last_run_ms: u64, outputs: Vec<RunResult>) -> Self {
        OutputList { last_run_ms: Some(last_run_ms), outputs: outputs }
    }
    pub fn last_run_ms(&self) -> Option<u64> { self.last_run_ms }
    pub fn outputs(&self) -> &Vec<RunResult> { &self.outputs }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    timeout_ms: u32,
    interval_ms: u32,
    down_threshold: u8,
}

impl Config {
    pub fn timeout_ms(&self) -> u32 { self.timeout_ms }
    pub fn interval_ms(&self) -> u32 { self.interval_ms }
    pub fn down_threshold(&self) -> u8 { self.down_threshold }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            timeout_ms: 1000 * 5,
            interval_ms: 1000 * 60 * 10,
            down_threshold: 3,
        }
    }
}

const CONFIG_FILE: &str = "config.json";
const SERVICE_FILE: &str = "services.json";
const OUTPUT_FILE: &str = "output.json";

pub fn load_config(path_str: &str) -> io::Result<Config> {
    let path = expanduser(path_str)?;

    ensure_directory_structure(&path)?;

    let config_file = fs::File::open(&path.join(CONFIG_FILE))?;
    let config: Config = serde_json::from_reader(config_file)?;

    Ok(config)
}

pub fn load_services(path_str: &str) -> io::Result<Vec<Service>> {
    let path = expanduser(path_str)?;

    ensure_directory_structure(&path)?;

    let service_file = fs::File::open(&path.join(SERVICE_FILE))?;
    let services: Vec<Service> = serde_json::from_reader(service_file)?;

    Ok(services)
}

pub fn load_output(path_str: &str) -> io::Result<OutputList> {
    let path = expanduser(path_str)?;
    
    ensure_directory_structure(&path)?;
    
    let output_file = fs::File::open(&path.join(OUTPUT_FILE))?;
    let output_list: OutputList = serde_json::from_reader(output_file)?;
    
    Ok(output_list)
}

fn ensure_directory_structure(path: &Path) -> io::Result<()> {
    fs::create_dir_all(path)?;

    let config_json = serde_json::to_string(&Config::default())?;
    let services_json = serde_json::to_string::<[Service]>(&[])?;
    let output_json = serde_json::to_string(&OutputList::default())?;

    write_if_missing(&path.join(CONFIG_FILE), &config_json)?;
    write_if_missing(&path.join(SERVICE_FILE), &services_json)?;
    write_if_missing(&path.join(OUTPUT_FILE), &output_json)?;

    Ok(())
}

/// Writes `contents` to `path` only if the file doesn't already exist.
fn write_if_missing(path: &Path, contents: &str) -> io::Result<()> {
    if !fs::exists(path)? {
        fs::write(path, contents)?;
    }
    Ok(())
}

pub fn write_out_file(path_str: &str, contents: &OutputList) -> io::Result<()> {
    let path = expanduser(path_str)?;

    ensure_directory_structure(&path)?;

    let file = fs::File::create(path.join(OUTPUT_FILE))?;
    serde_json::to_writer(file, contents)?;
    Ok(())
}