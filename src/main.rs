extern crate clap;
use clap::{crate_authors, crate_version, App, Arg};

mod lib;

static COMMANDS: &'static [&str] = &["init", "read", "write", "delete", "add", "list", "update"];

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
            Arg::with_name("value")
                .help("New value for updating of config or version attributes")
                .index(3),
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

    let command = matches.value_of("command").unwrap_or("help");
    let db = matches
        .value_of("database")
        .unwrap_or(lib::DEFAULT_DB_LOCATION);

    match command {
        "read" => lib::read_all(db).expect("read failed"),
        "write" => lib::write_all(db).expect("write failed"),
        "list" => match matches.value_of("entity") {
            Some("version") | Some("versions") => match matches.value_of("value") {
                Some(value) => lib::list_version(db, value).expect("listing of version and its configs failed"),
                None => lib::list_versions(db).expect("listing of versions failed"),
            },
            Some("config") | Some("configs") => lib::list_configs(db).expect("listing of configs failed"),
            Some(_) | None => println!(
                "You need to specify what you wanna list as a second argument (version/config)"
            ),
        },
        "delete" => match matches.value_of("entity") {
            Some("version") | Some("versions") => match matches.value_of("config-version") {
                Some(value) => lib::delete_version(db, value).expect("delete version failed"),
                None => println!("You need to specify version name by -v(--config-version)"),
            },
            Some("config") | Some("configs") => match matches.value_of("id"){
                Some(id) => lib::delete_by_id(db, id.parse::<u64>().expect("could not parse id"))
                    .expect("delete by id failed"),
                None => match matches.value_of("path") {
                    Some(path) => lib::delete_by_path(db, path).expect("delete by path failed"),
                    None => match matches.value_of("name") {
                        Some(name) => lib::delete_by_name(db, name).expect("delete by name failed"),
                        None => {
                            println!("You need either -i(--id) or -p(--path) for this command to work")
                        }
                    },
                },
            }
            Some(_) | None => println!(
                "version / config (you need to specify entity to delete)"
            ),
        },
        "update" => match matches.value_of("entity") {
            Some("version") => match matches.value_of("config-version") {
                Some(config_version) => match matches.value_of("value") {
                    Some(new_value) => lib::update_version(db, config_version, new_value)
                        .expect("update of version failed"),
                    None => println!( "You need to specify updated value for entity (next positional argument)"),
                },
                None => println!("You need to specify version you wanna update"),
            },
            Some("config") => match matches.value_of("path") {
                Some(path) => match matches.value_of("config-version") {
                    Some(config_version) => match matches.value_of("value") {
                        Some(new_value) => lib::update_config(db, path, config_version, new_value).expect("could not update config"),
                        None => println!( "You need to specify updated value for entity (next positional argument)"),
                    },
                    None => println!("You need to specify config version -v(--config-version) to match desired config")
                }
                None => println!("You need to specify old path to match our config")    
            },
            Some(_) | None => println!(
                "list / config (you need to specify entity to update)"
            ),
        },
        "init" => {
            lib::init_db(db).expect("fail init db");
            println!("db initialized");
        }
        "add" => match matches.value_of("path") {
            Some(path) => match matches.value_of("config-version") {
                Some(config_version) => {
                    lib::add_config(db, path, config_version).expect("add config failed");
                    println!("File added successfully");
                }
                None => println!("You need to specify version with -v(--config-version)"),
            },
            None => match matches.value_of("config-version") {
                Some(config_version) => {
                    lib::add_version(db, config_version).expect("add version failed");
                    println!("Version added successfully");
                }
                None => println!("You need to specify path with -p(--path) to add config or -v(--config-version) to add version"),
            },
        },
        "help" | _ => println!("unknown command!\noptions: {}", COMMANDS.join(", ")),
    }
}
