use rusqlite::{Connection, Result};
use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub mod entities;

#[allow(unused_imports)]
use entities::{Config, Entity, Version};

static DB_IS_FILE: bool = true;

/// returns db connection (either temporary in memory or in file)
/// determined by bool constant (mostly for development)
fn get_db() -> Connection {
    match DB_IS_FILE {
        true => Connection::open("rusty-sqlite.db").expect("Could not open db"),
        false => Connection::open_in_memory().unwrap(),
    }
}

/// db => real files
/// Writes into files from database
pub fn write_files() -> std::io::Result<()> {
    let db = get_db();
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
pub fn read_files() -> Result<()> {
    let db = get_db();
    let configs: Vec<Config> = fetch_configs(&db).expect("could not fetch data");
    println!("Real file data => db:");
    for config in configs {
        println!("{}", config.path);
        let new_data = fs::read_to_string(config.path).expect("could not read file in db");
        Config::update(&db, config.id, "data", &new_data)?
    }
    Ok(())
}

/// gets all the configs as a Vec<Config>
fn fetch_configs(db: &Connection) -> Result<Vec<Config>> {
    let configs: Vec<Config> = Config::select(&db, "id, path, data", |row| {
        let data: String = row.get(2)?;
        Ok(Config {
            id: row.get(0)?,
            path: row.get(1)?,
            data: data
                .split('\n')
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        })
    })?;
    Ok(configs)
}

/// testing config entity, create and fetch
#[test]
fn config_entity() -> Result<()> {
    let db = get_db();

    Config::table(&db)?;
    let test_config = Config {
        id: 1,
        path: "/tmp/test".to_string(),
        data: vec!["first line".to_string(), "second line".to_string()],
    };
    test_config.create(&db)?;
    let configs: Vec<Config> = Config::select(&db, "id, path, data", |row| {
        let data: String = row.get(2)?;
        Ok(Config {
            id: row.get(0)?,
            path: row.get(1)?,
            data: data
                .split('\n')
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
        })
    })?;
    assert_eq!(1, configs[0].id);
    assert_eq!("/tmp/test", configs[0].path);
    assert_eq!(
        vec!["first line".to_string(), "second line".to_string()],
        configs[0].data
    );
    Ok(())
}

/// testing version entity, create and fetch
#[test]
fn version_entity() -> Result<()> {
    let db = get_db();

    Version::table(&db)?;
    let test_version = Version {
        id: 1,
        name: "home".to_string(),
    };
    test_version.create(&db)?;
    let versions: Vec<Version> = Version::select(&db, "id, name", |row| {
        Ok(Version {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    assert_eq!(1, versions[0].id);
    assert_eq!("home", versions[0].name);
    Ok(())
}
