extern crate chrono;
extern crate chrono_tz;

use std::io::prelude::*;
use std::process::{Command, Stdio};
use crate::chrono::TimeZone;
use chrono_tz::Tz;

use chrono::offset::Local;

mod arg;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: arg::Args = arg::parse_arg();
    show_expiration(&(args.host_name + ":443"));
}

fn show_expiration(host_name: &str) {
    let inp = obtain_expiration(host_name);
    let pos = inp.rfind(' ').unwrap();
    let (time_str, tz_str) = inp.split_at(pos);
    let tz: Tz = tz_str.trim_start().parse().unwrap();

    let dt: chrono::DateTime<chrono_tz::Tz> = tz.datetime_from_str(
        time_str,
        "%b %d %H:%M:%S %Y"
    ).unwrap();

    let local_tz = Local::now().timezone();
    println!("{}", dt.with_timezone(&local_tz).to_rfc3339());
}

fn obtain_expiration(host_name: &str) -> String {
    let open_ssl_error = "Cannot exec openssl. Check installation.";

    let out = Command::new("openssl")
        .arg("s_client")
        .arg("-connect")
        .arg(host_name)
        .stdin(Stdio::null())
        .output()
        .expect(open_ssl_error)
        .stdout;

    let second_proc = Command::new("openssl")
        .arg("x509").arg("-noout").arg("-enddate")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect(open_ssl_error);

    second_proc.stdin.expect(open_ssl_error).write(&out).expect(open_ssl_error);
    let mut s = String::new();
    second_proc.stdout.unwrap().read_to_string(&mut s).expect(open_ssl_error);
    let split: Vec<&str> = s.split("notAfter=").collect();
    if split.len() != 2 {
        panic!("Output of openssl is unexpected: {}", s);
    }
    
    split[1].trim().to_string()
}

