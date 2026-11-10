use crate::Config;
use crate::config::{write_out_file, OutputList, RunResult, Service};
use crate::request_service::http_request;

pub fn execute(config: &Config, services: &[Service], previous_output: &OutputList, config_dir: &str, id: Option<&str>, print: bool, force: bool) -> Result<(), String> {
    let candidate_services: &[Service] = match id {
        Some(id) => services
            .iter()
            .find(|s| s.id() == id)
            .map(std::slice::from_ref)
            .ok_or_else(|| format!("No service with id '{}' found", id))?,
        None => services,
    };

    let mut run_results: Vec<RunResult> = previous_output.outputs()
        .iter()
        .filter(|r| services.iter().any(|s| s.id() == r.id()))
        .cloned()
        .collect();

    for service in candidate_services {
        let index = match run_results.iter().position(|r| r.id() == service.id()) {
            Some(index) => index,
            None => {
                run_results.push(RunResult::new(service.id().to_string(), 0));
                run_results.len() - 1
            }
        };
        let from_previous = &mut run_results[index];

        let res = http_request(service.url(), service.timeout_ms().unwrap_or(config.timeout_ms()));

        let is_up = matches!(res, Ok(true));
        if is_up {
            from_previous.reset_attempts();
        } else {
            from_previous.increment_attempts();
        }

        if print {
            let printable_status = match &res {
                Ok(false) if from_previous.failed_attempts() >= config.down_threshold()  => { "Down" },
                Ok(_) => { "Up" }
                Err(e) => &e.to_string(),
            };
            println!("{}: {}", service.name(), printable_status);
        }
    }

    write_out_file(config_dir, &OutputList::new(0, run_results)).map_err(|err| format!("Failed writing output file: {}", err))?;

    Ok(())
}
