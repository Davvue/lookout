use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    id: String,
    url: String,
    name: String,
    timeout_ms: Option<u32>,
    interval_ms: Option<u32>,
}

impl Service {
    pub fn new(
        id: String,
        url: String,
        name: String,
        timeout_ms: Option<u32>,
        interval_ms: Option<u32>,
    ) -> Self {
        Service {
            id,
            url,
            name,
            timeout_ms,
            interval_ms,
        }
    }
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn url(&self) -> &str {
        &self.url
    }
    pub fn timeout_ms(&self) -> Option<u32> {
        self.timeout_ms
    }
    pub fn interval_ms(&self) -> Option<u32> {
        self.interval_ms
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunResult {
    id: String,
    last_check_ms: Option<u64>,
    failed_attempts: u8,
}

impl RunResult {
    pub fn new(id: String, failed_attempts: u8) -> Self {
        Self {
            id,
            failed_attempts,
            last_check_ms: None,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn last_check_ms(&self) -> Option<u64> {
        self.last_check_ms
    }

    pub fn record(&mut self, success: bool, now_ms: u64) {
        self.last_check_ms = Some(now_ms);
        if success {
            self.failed_attempts = 0;
        } else {
            self.failed_attempts = self.failed_attempts.saturating_add(1);
        }
    }

    pub fn status(&self, down_threshold: u8) -> Status {
        match self.failed_attempts {
            0 => Status::Up,
            n if n >= down_threshold => Status::Down { attempts: n },
            n => Status::Failing {
                attempts: n,
                threshold: down_threshold,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Up,
    Failing { attempts: u8, threshold: u8 },
    Down { attempts: u8 },
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Up => write!(f, "Up"),
            Status::Failing {
                attempts,
                threshold,
            } => write!(f, "Failing ({attempts}/{threshold})"),
            Status::Down { attempts } => write!(f, "Down ({attempts} failed checks)"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct OutputList {
    last_run_ms: Option<u64>,
    outputs: Vec<RunResult>,
}

impl OutputList {
    pub fn new(last_run_ms: u64, outputs: Vec<RunResult>) -> Self {
        OutputList {
            last_run_ms: Some(last_run_ms),
            outputs,
        }
    }
    pub fn outputs(&self) -> &Vec<RunResult> {
        &self.outputs
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    timeout_ms: u32,
    interval_ms: u32,
    down_threshold: u8,
}

impl Config {
    pub fn timeout_ms(&self) -> u32 {
        self.timeout_ms
    }
    pub fn interval_ms(&self) -> u32 {
        self.interval_ms
    }
    pub fn down_threshold(&self) -> u8 {
        self.down_threshold
    }
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
