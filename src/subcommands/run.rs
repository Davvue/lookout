use crate::Config;
use crate::config::{write_out_file, OutputList, RunResult, Service};
use crate::request_service::http_request;

pub fn execute(config: &Config, services: &[Service], previous_output: &OutputList, config_dir: &str, id: &Option<String>, print: &bool, force: &bool) -> Result<(), String> {
    let candidate_services: &[Service] = match id {
        Some(id) => services
            .iter()
            .find(|s| s.id() == id)
            .map(std::slice::from_ref)
            .ok_or_else(|| format!("No service with id '{}' found", id))?,
        None => services,
    };

    let mut run_results: Vec<RunResult> = Vec::with_capacity(candidate_services.len());

    for service in candidate_services {
        let mut from_previous = previous_output.outputs()
            .iter()
            .find(|i| i.id() == service.id())
            .cloned()
            .unwrap_or_else(|| RunResult::new(service.id().to_string(), 0));

        let res = http_request(service.url(), service.timeout_ms().unwrap_or(config.timeout_ms()));

        if *print {
            let printable_status = match &res {
                Ok(val) => if *val { "Up" } else { "Down" },
                Err(e) => &e.to_string(),
            };
            println!("{}: {}", service.name(), printable_status);
        }

        if res.unwrap_or(false) {
            from_previous.increment_attempts();
        }

        run_results.push(from_previous.clone());
    }

    match write_out_file(config_dir, &OutputList::new(0, run_results)) {
        Ok(_) => (),
        Err (err) => return Err(format!("Failed writing output file: {}", err)),
    }

    Ok(())
}
