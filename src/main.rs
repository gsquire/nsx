extern crate curl;
extern crate json;

use std::env;
use std::str;

use curl::easy::Easy;

const GOOGLE_DNS: &'static str = "https://dns.google.com/resolve?name=";

// Just pull out the IP addresses for now.
fn unpack_json(data: &[u8]) {
    let response = str::from_utf8(data).unwrap();
    let j = json::parse(response).unwrap();
    println!("{}", j["Answer"]);
}

// Call out to Google's public DNS over HTTPS endpoint.
fn run_query(domain_name: String) {
    let full_url = format!("{}{}", GOOGLE_DNS, domain_name);

    let mut easy = Easy::new();
    easy.url(&full_url).unwrap();
    easy.write_function(|data| {
        unpack_json(data);
        Ok(data.len())
    }).unwrap();

    easy.perform().unwrap();
}

fn main() {
    let domain_name: String;
    if let Some(h) = env::args().nth(1) {
        domain_name = h;
    } else {
        println!("must supply a domain name to query...");
        return;
    }

    run_query(domain_name);
}
