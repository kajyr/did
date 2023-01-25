extern crate chrono;
extern crate clap;

use chrono::Local;
use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

/**
 * Returns a file tro write to.
 * If it's specified in the args it returns it,
 * otherwise return a file in the home directory
 * otherwise a file in the current directory
 */
fn get_file(from_args: Option<String>) -> Result<PathBuf, std::io::Error> {
    let from = match from_args {
        Some(val) => Path::new(&val).to_path_buf(),
        None => {
            let cur_dir = env::current_dir()?;
            let home_path = dirs::home_dir().unwrap_or(cur_dir);
            let full = home_path.join("did.txt");
            full
        }
    };
    Ok(from)
}

struct Line {
    date_fmt: String,
    ticket: String,
    message: String,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}{}", self.date_fmt, self.ticket, self.message)
    }
}

fn main() {
    let matches = App::new("did")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Helps me remember what I did")
        .author("Carlo <carlo.panzi@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("The descriptive string of what you did")
                .required(true)
                .multiple(true)
                .index(1),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Specify the file to write to.\nDefaults to did.txt in your home folder.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("ticket")
                .short("t")
                .long("ticket")
                .value_name("ID")
                .help("Specify a ticket number")
                .takes_value(true),
        )
        .get_matches();

    let ticket = matches.value_of("ticket").unwrap_or("").to_string();
    let dids: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let message = dids.join(" ").to_string();
    let date_fmt = Local::now().format("[%Y-%m-%d %H:%M:%S]").to_string();

    let line = Line {
        date_fmt,
        ticket,
        message,
    };

    let file_name_arg = match matches.value_of("file") {
        Some(val) => Some(val.to_string()),
        None => None,
    };

    let did_file = get_file(file_name_arg).expect("Unable to determine the output file");

    let mut file = File::options()
        .append(true)
        .create(true)
        .open(did_file)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
    }

    println!("{}", line);
}
