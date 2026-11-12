use crate::config::Service;
use crate::store::Store;

pub fn execute(
    store: &dyn Store,
    url: String,
    id: String,
    name: Option<String>,
    timeout_ms: Option<u32>,
    interval_ms: Option<u32>,
) -> Result<(), String> {
    let mut existing_services = store
        .services()
        .map_err(|e| format!("Failed to load services: {}", e))?;

    if existing_services.iter().any(|s| s.id() == id) {
        return Err(format!("Service with id '{}' already exists", id));
    }

    let new_service = Service::new(id.clone(), url, name.unwrap_or(id.clone()), timeout_ms, interval_ms);

    existing_services.push(new_service);
    println!("Service with id '{}' added", id);

    store
        .save_services(&existing_services)
        .map_err(|e| format!("Failed to save services: {}", e))
}
