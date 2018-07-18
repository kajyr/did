extern crate chrono;
extern crate clap;

use chrono::Local;
use clap::{App, Arg};
use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let matches = App::new("did")
        .version("0.1.0")
        .about("Helps me remember what I did")
        .author("Carlo <carlo.panzi@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let date = Local::now();
    let home_path = env::home_dir().unwrap();
    let did_file = home_path.join("did.txt");

    let date_str = date.format("[%Y-%m-%d %H:%M:%S]");
    let did_str = matches.value_of("INPUT").unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(did_file)
        .unwrap();

    let line = format!("{} - {}", date_str, did_str);

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("{}", line);
}
