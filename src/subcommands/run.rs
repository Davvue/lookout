use crate::config::{OutputList, RunResult};
use crate::request_service::http_request;
use crate::store::Store;

pub fn execute(store: &dyn Store, id: Option<&str>, print: bool, force: bool) -> Result<(), String> {
    let config = store.config().map_err(|err| format!("Failed to load config: {}", err))?;
    let services = store.services().map_err(|err| format!("Failed to load services: {}", err))?;

    let candidate_services = match id {
        Some(id) => services
            .iter()
            .find(|s| s.id() == id)
            .map(std::slice::from_ref)
            .ok_or_else(|| format!("No service with id '{}' found", id))?,
        None => services.as_slice(),
    };

    let previous_output = store.output()
        .map_err(|err| format!("Failed loading previous output: {}", err))?;

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
        let result = &mut run_results[index];

        let res = http_request(service.url(), service.timeout_ms().unwrap_or(config.timeout_ms()));
        result.record(matches!(res, Ok(true)));

        if print {
            let status = result.status(config.down_threshold());
            match &res {
                Err(err) => println!("{}: {} - {}", service.name(), status, err),
                Ok(_) => println!("{}: {}", service.name(), status),
            }
        }
    }

    store.save_output(&OutputList::new(0, run_results))
        .map_err(|err| format!("Failed writing output: {}", err))?;

    Ok(())
}
