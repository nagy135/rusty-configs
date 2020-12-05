extern crate clap;
use clap::{App, Arg};

mod lib;

fn main() {
    let matches = App::new("Rusty Configs")
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about("Synchronizes configs across devices with sqlite")
        .arg(
            Arg::with_name("command")
                .help("Command to run")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let command = matches.value_of("command").unwrap_or("read");
    println!("Value for command: {}", command);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    match command {
        "read" => lib::read_files().expect("read failed"),
        "write" => lib::write_files().expect("write failed"),
        _ => panic!("Unknown option"),
    }
}
