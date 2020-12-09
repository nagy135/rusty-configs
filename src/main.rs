extern crate clap;
use clap::{crate_authors, crate_version, App, Arg};

mod lib;

static COMMANDS: &'static [&str] = &["read", "write", "delete", "add"];

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
            Arg::with_name("name")
                .long("name")
                .short("n")
                .takes_value(true)
                .help("Name of target config (for path /tmp/test it is -n test)"),
        )
        .arg(
            Arg::with_name("id")
                .long("id")
                .short("i")
                .takes_value(true)
                .help("Id of target config"),
        )
        .arg(
            Arg::with_name("config_version")
                .short("g")
                .long("config_version")
                .takes_value(true)
                .help("Version of config (name of the system where it is)"),
        )
        .get_matches();

    let command = matches.value_of("command").unwrap_or("read");

    match command {
        "read" => lib::read_all().expect("read failed"),
        "write" => lib::write_all().expect("write failed"),
        "delete" => match matches.value_of("id") {
            Some(id) => lib::delete_by_id(id.parse::<u64>().expect("could not parse id"))
                .expect("delete by id failed"),
            None => match matches.value_of("path") {
                Some(path) => lib::delete_by_path(path).expect("delete by path failed"),
                None => match matches.value_of("name") {
                    Some(name) => lib::delete_by_name(name).expect("delete by name failed"),
                    None => {
                        panic!("You need either -i(--id) or -p(--path) for this command to work")
                    }
                },
            },
        },
        "init" => {
            lib::init_db().expect("fail init db");
            println!("db initialized");
        }
        "add" => match matches.value_of("path") {
            Some(path) => match matches.value_of("config_version") {
                Some(config_version) => {
                    lib::add(path, config_version).expect("add failed");
                    println!("File added successfully");
                }
                None => println!("You need to specify version with -g(--config_version)"),
            },
            None => println!("You need to specify path with -p(--path)"),
        },
        _ => println!("unknown command!\noptions: {}", COMMANDS.join(", ")),
        // _ => lib::read_all().expect("read failed"),
    }
}
