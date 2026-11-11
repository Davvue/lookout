use crate::config::Service;
use crate::store::Store;

pub fn execute(store: &dyn Store) -> Result<(), String> {
    let services = store.services().map_err(|err| format!("Failed to load services: {}", err))?;

    if services.is_empty() {
        println!("No services configured. Use `add` to add one.");
        return Ok(());
    }

    let headers = ("NAME", "ID", "URL");

    let width = |header: &str, field: fn(&Service) -> &str| {
        services
            .iter()
            .map(|s| field(s).chars().count())
            .chain(std::iter::once(header.len()))
            .max()
            .unwrap_or(0)
    };
    let name_width = width(headers.0, Service::name);
    let id_width = width(headers.1, Service::id);

    println!("{:<name_width$}  {:<id_width$}  {}", headers.0, headers.1, headers.2);
    for service in &services {
        println!(
            "{:<name_width$}  {:<id_width$}  {}",
            service.name(),
            service.id(),
            service.url()
        );
    }
    Ok(())
}