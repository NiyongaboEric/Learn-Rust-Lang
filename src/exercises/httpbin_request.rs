use reqwest;
use std::collections::HashMap;
use std::error::Error;

#[tokio::main]
pub async fn ipaddres() -> Result<i32, Box<dyn Error>> {
	let response = reqwest::get("https://httpbin.org/ip")
			.await?
			.json::<HashMap<String, String>>()
			.await?;
	// println!("My PC IP Addres is {:#?}", response);
	// Ok(());
	// (e) => println!('e')
	// "My PC IP Addres is {:#?}", response
	Ok((c_string) => Ok("My PC IP Addres is {:#?}", response))

	Err("Error IP address is not fetched".into());
}
