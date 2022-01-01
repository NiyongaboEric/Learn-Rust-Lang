use reqwest;
use std::error::Error;


#[tokio::main]
pub async fn ipaddres() -> Result<String, Box<dyn Error>> {
	let url: &str = "https://httpbin.org/ip";
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header("Accept", "text/plain")
		.send()
		.await?
		.text()
		.await?;
	Ok(res)
}
