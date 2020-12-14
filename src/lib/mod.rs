use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub mod entities;

#[allow(unused_imports)]
use entities::{Config, Entity, Version};

pub static DEFAULT_DB_LOCATION: &'static str = "db.sqlite";

static DB_IS_FILE: bool = true;

/// returns db connection (either temporary in memory or in file)
/// determined by bool constant (mostly for development)
fn get_db(db: &str) -> Connection {
    match DB_IS_FILE {
        true => Connection::open(db).expect("Could not open db"),
        false => Connection::open_in_memory().unwrap(),
    }
}

/// initializes tables of database
pub fn init_db(db: &str) -> Result<()> {
    let db = get_db(db);
    Config::table(&db)?;
    Version::table(&db)?;
    Ok(())
}

/// updates path location of config (match by old one)
pub fn update_config(db: &str, path: &str, version: &str, new_value: &str) -> std::io::Result<()> {
    let db = get_db(db);
    let version: Vec<Version> = Version::select_where(&db, &format!("name='{}'", version))
        .expect("could not select version");
    if version.len() == 0 {
        panic!("No version with given name exists");
    }
    let matched_configs: Vec<Config> = Config::select_where(
        &db,
        &format!("path='{}' AND version_id={}", path, version[0].id),
    )
    .expect("Could not select any configs with condition");
    if matched_configs.len() == 0 {
        panic!("No config with given criteria exists");
    }

    let options = ["path", "version"];
    let splitted: Vec<&str> = new_value.split("=").collect();
    let column = splitted[0];
    let value = splitted[1];

    if !options.contains(&column) {
        panic!("Unknown column to update, options: path, version");
    }
    if column == "version" {
        let version: Vec<Version> = Version::select_where(&db, &format!("name='{}'", value))
            .expect("could not select version");
        let version_value: String = version[0].id.to_string();
        for config in matched_configs {
            Config::update(&db, config.id, "version_id", &version_value)
                .expect("config version update failed");
        }
    } else {
        for config in matched_configs {
            Config::update(&db, config.id, column, value).expect("config path update failed");
        }
    }
    println!("Config {} update successfull", column);
    Ok(())
}
/// updates name of version (match by old name)
pub fn update_version(db: &str, name: &str, new_name: &str) -> Result<()> {
    let db = get_db(db);
    let versions: Vec<Version> =
        Version::select_where(&db, &format!("name='{}'", name)).expect("could not select version");
    if versions.len() == 0 {
        panic!("No version matches criteria");
    }
    for version in versions {
        Version::update(&db, version.id, "name", &new_name)?
    }
    println!("Version name updated {} => {}", name, new_name);
    Ok(())
}

/// delete config by its id
pub fn delete_by_id(db: &str, id: u64) -> std::io::Result<()> {
    let db = get_db(db);
    Config::delete(&db, "id", &id.to_string(), "=").expect("Delete by id failed");
    Ok(())
}

/// delete config by its full path
pub fn delete_by_path(db: &str, path: &str) -> std::io::Result<()> {
    let db = get_db(db);
    Config::delete(&db, "path", &format!("\"{}\"", path), "=").expect("Delete by path failed");
    Ok(())
}

/// delete config by its name (last token separated by slash)
pub fn delete_by_name(db: &str, name: &str) -> std::io::Result<()> {
    let db = get_db(db);
    Config::delete(&db, "path", &format!("'%{}'", name), " LIKE ").expect("Delete by name failed");
    Ok(())
}

/// adds new config to database
pub fn add_config(db: &str, path: &str, version: &str) -> std::io::Result<()> {
    let db = get_db(db);
    let file_lines = fs::read_to_string(path).expect("could not read file in db");
    let new_id: i32 = Config::next_id(&db).expect("could not fetch next id");

    let new_config = Config {
        id: new_id,
        version_id: version.parse::<i32>().unwrap(),
        path: path.to_string(),
        data: file_lines
            .split("\n")
            .map(|e| e.to_string())
            .collect::<Vec<String>>(),
    };
    new_config
        .create(&db)
        .expect("could not create config in db");
    Ok(())
}

/// adds new version to database
pub fn add_version(db: &str, name: &str) -> std::io::Result<()> {
    let db = get_db(db);
    let new_id: i32 = Version::next_id(&db).expect("could not fetch next id");

    let new_version = Version {
        id: new_id,
        name: name.to_string(),
    };
    new_version
        .create(&db)
        .expect("could not create version in db");
    Ok(())
}

/// db => real files
/// Writes into files from database
pub fn write_all(db: &str) -> std::io::Result<()> {
    let db = get_db(db);
    let configs: Vec<Config> = fetch_configs(&db).expect("could not fetch data");
    println!("db => real file contents:");
    for config in configs {
        println!("{}", config.path);
        let mut file = File::create(config.path)?;
        for line in config.data {
            file.write(format!("{}\n", line).as_bytes())?;
        }
    }
    Ok(())
}

/// real files => db
/// Reads actual file contents and updates their data in database
pub fn read_all(db: &str) -> Result<()> {
    let db = get_db(db);
    let configs: Vec<Config> = fetch_configs(&db).expect("could not fetch data");
    println!("Real file data => db:");
    for config in configs {
        println!("{}", config.path);
        let new_data = fs::read_to_string(config.path).expect("could not read file in db");
        Config::update(&db, config.id, "data", &new_data)?
    }
    Ok(())
}

/// lists line separated list of versions stored in db
pub fn list_versions(db: &str) -> Result<()> {
    let _db = get_db(db);
    Ok(())
}

/// prints line of tree list
fn tree_item(index: usize, total_len: usize, shift_len: usize, item: &str) -> String {
    let mut tree_branch = "├──";
    if (index == 0 && total_len == 1) || index == total_len - 1 {
        tree_branch = "└──";
    }
    format!("{}{} {}", " ".repeat(shift_len), tree_branch, item)
}

/// lists line separated list of configs stored in db
pub fn list_configs(db: &str) -> Result<()> {
    let db = get_db(db);
    let configs: Vec<Config> = fetch_configs(&db).expect("could not fetch data");
    if configs.len() == 0 {
        println!("No configs in db");
    } else {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for config in configs {
            let version_name = Version::find(&db, config.version_id)?.name;
            if let Some(version_vec) = map.get_mut(&version_name) {
                version_vec.push(config.path);
            } else {
                map.insert(version_name, vec![config.path]);
            }
        }
        for (version_name, config_vec) in map {
            println!("================");
            println!("{}", version_name);
            for (i, config_path) in config_vec.iter().enumerate() {
                println!(
                    "{}",
                    tree_item(i, config_vec.len(), version_name.len() + 1, config_path)
                );
            }
        }
    }
    Ok(())
}

/// gets all the configs as a Vec<Config>
fn fetch_configs(db: &Connection) -> Result<Vec<Config>> {
    let configs: Vec<Config> = Config::all(&db)?;
    Ok(configs)
}

/// testing version entity, create and fetch
#[test]
fn version_entity() -> Result<()> {
    let db = get_db(DEFAULT_DB_LOCATION);

    // setup
    Version::table(&db)?;
    let test_version = Version {
        id: 1,
        name: "home".to_string(),
    };
    test_version.create(&db)?;

    // all
    let versions: Vec<Version> = Version::all(&db)?;
    assert_eq!(1, versions[0].id);
    assert_eq!("home", versions[0].name);

    // find
    let version: Version = Version::find(&db, 1)?;
    assert_eq!(1, version.id);
    assert_eq!("home".to_string(), version.name);

    // update
    Version::update(&db, version.id, "name", "work")?;
    let updated_version: Version = Version::find(&db, 1)?;
    assert_eq!(1, updated_version.id);
    assert_eq!("work".to_string(), updated_version.name);

    Ok(())
}

/// testing config entity, create and fetch
#[test]
fn config_entity() -> Result<()> {
    let db = get_db(DEFAULT_DB_LOCATION);

    // setup
    Config::table(&db)?;
    let test_config = Config {
        id: 1,
        version_id: 1,
        path: "/tmp/test".to_string(),
        data: vec!["first line".to_string(), "second line".to_string()],
    };
    test_config.create(&db)?;

    // all
    let configs: Vec<Config> = Config::all(&db)?;
    assert_eq!(1, configs[0].id);
    assert_eq!("/tmp/test", configs[0].path);
    assert_eq!(
        vec!["first line".to_string(), "second line".to_string()],
        configs[0].data
    );

    // find
    let config: Config = Config::find(&db, 1)?;
    assert_eq!(1, config.id);
    assert_eq!(1, config.version_id);
    assert_eq!("/tmp/test".to_string(), config.path);

    // update
    Config::update(&db, config.id, "path", "/tmp/test2")?;
    let updated_config: Config = Config::find(&db, 1)?;
    assert_eq!(1, updated_config.id);
    assert_eq!("/tmp/test2".to_string(), updated_config.path);

    Ok(())
}
