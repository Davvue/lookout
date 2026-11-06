use crate::config::Service;

pub fn execute(services: Vec<Service>) {
    if services.is_empty() {
        println!("No services configured. Use `add` to add one.");
        return;
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
}