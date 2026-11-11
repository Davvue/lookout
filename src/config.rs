use serde::{Deserialize, Serialize};

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
