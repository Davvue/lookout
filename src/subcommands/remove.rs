use crate::store::Store;

pub fn execute(store: &dyn Store, id: &str) -> Result<(), String> {
    let mut services = store.services().map_err(|err| format!("Failed loading services: {}", err.to_string()))?;

    let before = services.len();
    services.retain(|s| s.id() != id);

    if services.len() != before {
        return Err(format!("Service with id '{}' doesn't exist", id));
    }

    store.save_services(&services).map_err(|err| format!("Failed saving services: {}", err))
}
