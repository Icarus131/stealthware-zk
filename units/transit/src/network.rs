use reqwest::blocking::Client;
use std::error::Error;

pub fn send_proof(data: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .post("http://127.0.01:8080/")
        .body(data.to_string())
        .send()?;

    if response.status().is_success() {
        println!("Data sent successfully.");
    } else {
        println!("Failed to send data.");
    }

    Ok(())
}
