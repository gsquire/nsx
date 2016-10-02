extern crate curl;
extern crate docopt;
extern crate json;
extern crate rustc_serialize;

use std::str;

use curl::easy::Easy;

use docopt::Docopt;

const PADDING: u16 = 4;
const GOOGLE_DNS: &'static str = "https://dns.google.com/resolve?name=";

const USAGE: &'static str = "
Usage:
    nsx [options] <domain>
    nsx (-h | --help)

Options:
    -d, --disable          Disable DNSSEC validation.
    -e, --edns-subnet ARG  An optional subnet formed as IP address with a subnet mask.
    -h, --help             Print this help message.
    -r, --random-pad ARG   A discarded value to pad requests with random data.
    -t, --type ARG         DNS query type, can be a number [1, 65535] or a canonical string.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_domain: String,
    flag_disable: bool,
    flag_edns_subnet: Option<String>,
    flag_random_pad: Option<String>,
    flag_type: Option<String>,
}

// Just pull out the IP addresses for now.
fn unpack_json(data: &[u8]) {
    let response = str::from_utf8(data).map_err(|e| {
        println!("error converting to string: {}", e);
    });
    let j = json::parse(response.unwrap());
    match j {
        Ok(ans) => {
            let pretty = json::stringify_pretty(ans["Answer"].clone(), PADDING);
            println!("{}", pretty);
        },
        Err(e) => { println!("error: {}", e); },
    }
}

// Make the URL that we will use to query the API with.
fn make_url(args: &Args) -> String {
    let mut full_url = format!("{}{}", GOOGLE_DNS, args.arg_domain);

    if args.flag_disable {
        full_url.push_str("&cd=true");
    }

    if let Some(ref e) = args.flag_edns_subnet {
        full_url.push_str(&format!("&edns_client_subnet={}", e));
    }

    if let Some(ref r) = args.flag_random_pad {
        full_url.push_str(&format!("&random_padding={}", r));
    }

    if let Some(ref t) = args.flag_type {
        full_url.push_str(&format!("&type={}", t));
    }

    full_url
}

// Call out to Google's public DNS over HTTPS endpoint.
fn run_query(args: Args) -> Result<(), curl::Error> {
    let full_url = make_url(&args);

    let mut easy = Easy::new();
    try!(easy.url(&full_url));
    try!(easy.write_function(|data| {
        unpack_json(data);
        Ok(data.len())
    }));

    try!(easy.perform());
    Ok(())
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let _res = run_query(args);
}
