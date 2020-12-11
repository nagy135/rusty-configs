extern crate clap;
use clap::{crate_authors, crate_version, App, Arg};

mod lib;

static COMMANDS: &'static [&str] = &["read", "write", "delete", "add", "list"];

fn main() {
    let matches = App::new("Rusty Configs")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Synchronizes configs across devices with sqlite")
        .arg(
            Arg::with_name("command")
                .help(&format!("Command to run. Options: {}", COMMANDS.join(", ")))
                .index(1),
        )
        .arg(
            Arg::with_name("entity")
                .help("Specifies target entity of command")
                .index(2),
        )
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
            Arg::with_name("config-version")
                .short("v")
                .long("config-version")
                .takes_value(true)
                .help("Version of config (name of the system where it is)"),
        )
        .arg(
            Arg::with_name("database")
                .long("db")
                .short("d")
                .takes_value(true)
                .help("Sqlite database file location)"),
        )
        .get_matches();

    let command = matches.value_of("command").unwrap_or("read");
    let db = matches
        .value_of("database")
        .unwrap_or(lib::DEFAULT_DB_LOCATION);

    match command {
        "read" => lib::read_all(db).expect("read failed"),
        "write" => lib::write_all(db).expect("write failed"),
        "list" => match matches.value_of("entity") {
            Some("version") => lib::list_versions(db).expect("listing of versions failed"),
            Some("config") => lib::list_configs(db).expect("listing of configs failed"),
            Some(_) | None => println!(
                "You need to specify what you wanna list as a second argument (version/config)"
            ),
        },
        "delete" => match matches.value_of("id") {
            Some(id) => lib::delete_by_id(db, id.parse::<u64>().expect("could not parse id"))
                .expect("delete by id failed"),
            None => match matches.value_of("path") {
                Some(path) => lib::delete_by_path(db, path).expect("delete by path failed"),
                None => match matches.value_of("name") {
                    Some(name) => lib::delete_by_name(db, name).expect("delete by name failed"),
                    None => {
                        panic!("You need either -i(--id) or -p(--path) for this command to work")
                    }
                },
            },
        },
        "init" => {
            lib::init_db(db).expect("fail init db");
            println!("db initialized");
        }
        "add" => match matches.value_of("path") {
            Some(path) => match matches.value_of("config-version") {
                Some(config_version) => {
                    lib::add(db, path, config_version).expect("add failed");
                    println!("File added successfully");
                }
                None => println!("You need to specify version with -g(--config-version)"),
            },
            None => println!("You need to specify path with -p(--path)"),
        },
        _ => println!("unknown command!\noptions: {}", COMMANDS.join(", ")),
        // _ => lib::read_all(db).expect("read failed"),
    }
}
