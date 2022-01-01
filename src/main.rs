#[path="./exercises/httpbin_request.rs"]
mod httpbin_request;


fn main() {
	match httpbin_request::ipaddres() {
		Ok(result) => println!("Yeah! Your Ip address is: {}", result),
		Err(e) => eprintln!("Oop!, we can't find your ip address :( \n  {}", e),
	}
}
