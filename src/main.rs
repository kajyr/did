extern crate chrono;
extern crate clap;

use chrono::Local;
use clap::{App, Arg};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let matches = App::new("did")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Helps me remember what I did")
        .author("Carlo <carlo.panzi@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("ticket")
                .short("t")
                .long("ticket")
                .value_name("FILE")
                .help("Specify a ticket number")
                .takes_value(true),
        )
        .get_matches();

    let date = Local::now();
    let home_path = env::home_dir().unwrap();
    let did_file = home_path.join("did.txt");

    let ticket = match matches.value_of("ticket") {
        Some(t) => format!("{} ", t),
        None => "".to_string(),
    };

    let date_str = date.format("[%Y-%m-%d %H:%M:%S]");
    let did_str = matches.value_of("INPUT").unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(did_file)
        .unwrap();

    let line = format!("{} - {}{}", date_str, ticket, did_str);

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("{}", line);
}
