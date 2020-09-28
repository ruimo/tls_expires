extern crate clap;

use super::VERSION;
use clap::{App, Arg};

pub struct Args {
    pub host_name: String,
    pub server_name: Option<String>
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
        .arg(Arg::with_name("server_name")
             .help("Server name to check the TLS expiration. You need to specify the server name if the host name is different than the host name of specified in the certificate. This would be the case when using CNAME for example. Ex: www.ruimo.com")
             .long("servername")
             .takes_value(true)
        )
        ;
        
    let matches = app.get_matches();
    Args {
        host_name: matches.value_of("host_name").unwrap().to_string(),
        server_name: matches.value_of("server_name").map(|s| s.to_string()),
    }
}
