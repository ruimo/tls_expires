extern crate clap;

use super::VERSION;
use clap::{App, Arg};

pub struct Args {
    pub host_name: String,
}

pub fn parse_arg() -> Args {
    let app = App::new("ssl_expires")
        .version(VERSION)
        .author("Shisei Hanai<ruimo.uno@gmail.com>")
        .about("Show TLS expiration time")
        .arg(Arg::with_name("host_name")
             .help("Host name to check the TLS expiration. Ex: www.ruimo.com")
             .required(true)
        )
        ;
        
    let matches = app.get_matches();
    Args {
        host_name: matches.value_of("host_name").unwrap().to_string(),
    }
}
