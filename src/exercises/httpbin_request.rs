use reqwest;
use std::error::Error;
use std::collections::HashMap;


#[tokio::main]
pub async fn ipaddres() -> Result<(), Box<dyn Error>> {
	let url: &str = "https://httpbin.org/ip";
	let client = reqwest::Client::new();
	let res = client
		.get(url)
		.header("Accept", "text/plain")
		.send()
		.await?
		.json::<HashMap<String, String>>()
		.await?;
	println!("My PC IP Addres is {:#?}", res);
	Ok(())
}
