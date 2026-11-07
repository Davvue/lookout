use crate::Config;
use crate::config::Service;
use crate::request_service::http_request;

pub fn execute(config: &Config, services: &[Service], id: &Option<String>, out_file: &Option<String>, print: &bool, force: &bool) -> Result<(), String> {
    let services_to_check: &[Service] = match id {
        Some(id) => services
            .iter()
            .find(|s| s.id() == id)
            .map(std::slice::from_ref)
            .ok_or_else(|| format!("No service with id '{}' found", id))?,
        None => services,
    };

    for service in services_to_check {
        let res = http_request(service.url(), service.timeout_ms().unwrap_or(config.timeout_ms()));
        println!("{:?}: {:?}", service.id(), res);
    }

    Err("Not implemented".to_string())
}
