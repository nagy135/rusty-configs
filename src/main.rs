extern crate clap;
use clap::{crate_authors, crate_version, App, Arg};

mod lib;

fn main() {
    let matches = App::new("Rusty Configs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Synchronizes configs across devices with sqlite")
        .arg(Arg::with_name("command").help("Command to run").index(1))
        .arg(
            Arg::with_name("path")
                .long("path")
                .short("p")
                .takes_value(true)
                .help("Path of target config"),
        )
        .arg(
            Arg::with_name("id")
                .long("id")
                .short("i")
                .takes_value(true)
                .help("Id of target config"),
        )
        .arg(
            Arg::with_name("verbosity")
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
    match matches.occurrences_of("verbosity") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    match command {
        "read" => lib::read_all().expect("read failed"),
        "write" => lib::write_all().expect("write failed"),
        "delete" => match matches.value_of("id") {
            Some(id) => lib::delete_by_id(id.parse::<u64>().expect("could not parse id"))
                .expect("delete by id failed"),
            None => match matches.value_of("path") {
                Some(path) => lib::delete_by_path(path).expect("delete by path failed"),
                None => panic!("You need either -i(--id) or -p(--path) for this command to work"),
            },
        },
        // _ => panic!("Unknown option"),
        _ => lib::read_all().expect("read failed"),
    }
}
