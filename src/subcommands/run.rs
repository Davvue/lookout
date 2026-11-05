use crate::Config;

pub fn execute(config: &Config) -> Result<(), String> {
    println!("{:?}", config);
    Err("Not implemented".to_string())
}
