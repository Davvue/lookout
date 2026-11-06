use crate::Config;

pub fn execute(config: &Config, id: &Option<String>, out_file: &Option<String>, print: &bool, force: &bool) -> Result<(), String> {
    println!("{:?}", config);
    println!("id: {:?}; out_file: {:?}; print: {:?}; force: {:?}", id, out_file, print, force);
    Err("Not implemented".to_string())
}
