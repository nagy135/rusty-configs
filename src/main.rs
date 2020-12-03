#[allow(unused_imports)]
use rusqlite::{params, Connection, Result, NO_PARAMS};
use std::fs::File;
use std::io::prelude::*;

mod entities;

#[allow(unused_imports)]
use entities::{Config, Entity, Version};

static DB_IS_FILE: bool = true;

fn main() -> Result<()> {
    write_files().expect("Writing of files unsuccessful");
    Ok(())
}

fn get_db() -> Connection {
    match DB_IS_FILE {
        true => Connection::open("rusty-sqlite.db").expect("Could not open db"),
        false => Connection::open_in_memory().unwrap(),
    }
}

fn write_files() -> std::io::Result<()> {
    let db = get_db();
    let configs: Vec<Config> = fetch_data(&db).expect("could not fetch data");
    for config in configs.into_iter() {
        println!("writing into {}", config.path);
        let mut file = File::create(config.path)?;
        for line in config.data {
            file.write(format!("{}\n", line).as_bytes())?;
        }
    }
    Ok(())
}

fn read_files() -> Result<()> {
    let db = get_db();
    Ok(())
}

fn fetch_data(db: &Connection) -> Result<Vec<Config>> {
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
