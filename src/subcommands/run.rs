use crate::config::{OutputList, RunResult};
use crate::request_service::http_request;
use crate::store::Store;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn execute(
    store: &dyn Store,
    id: Option<&str>,
    print: bool,
    force: bool,
) -> Result<(), String> {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;

    let config = store
        .config()
        .map_err(|err| format!("Failed to load config: {}", err))?;
    let services = store
        .services()
        .map_err(|err| format!("Failed to load services: {}", err))?;
    let previous_output = store
        .output()
        .map_err(|err| format!("Failed loading previous output: {}", err))?;

    let candidate_services = match id {
        Some(id) => services
            .iter()
            .find(|s| s.id() == id)
            .map(std::slice::from_ref)
            .ok_or_else(|| format!("No service with id '{}' found", id))?,
        None => services.as_slice(),
    };

    let mut run_results: Vec<RunResult> = previous_output
        .outputs()
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

        let remaining = remaining_ms(
            now_ms,
            result.last_check_ms().unwrap_or(0),
            service.interval_ms().unwrap_or(config.interval_ms())
        );

        if !force
            && remaining > 0
        {
            if print { println!("{}: skipped (next check due in {})", service.name(), format_duration(remaining)); }
            continue;
        }

        let res = http_request(
            service.url(),
            service.timeout_ms().unwrap_or(config.timeout_ms()),
        );
        result.record(matches!(res, Ok(true)), now_ms);

        if print {
            let status = result.status(config.down_threshold());
            match &res {
                Err(err) => println!("{}: {} - {}", service.name(), status, err),
                Ok(_) => println!("{}: {}", service.name(), status),
            }
        }
    }

    store
        .save_output(&OutputList::new(now_ms, run_results))
        .map_err(|err| format!("Failed writing output: {}", err))?;

    Ok(())
}

fn remaining_ms(now_ms: u64, last_check_ms: u64, interval_ms: u32) -> u64 {
    let elapsed = now_ms.saturating_sub(last_check_ms);
    u64::from(interval_ms).saturating_sub(elapsed)
}

fn format_duration(remaining: u64) -> String {
    let total_secs = remaining.div_ceil(1000);
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{}m {:2}s", mins, secs)
}