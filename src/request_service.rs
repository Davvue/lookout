use std::time::Duration;

pub fn http_request(request_url: &str, timeout_ms: u32) -> Result<bool, reqwest::Error> {
    let timeout = Duration::from_millis(timeout_ms as u64);

    let response = reqwest::blocking::Client::new()
        .get(request_url)
        .timeout(timeout)
        .send()?;

    if !response.status().is_success() {
        return Ok(false);
    }

    Ok(true)
}