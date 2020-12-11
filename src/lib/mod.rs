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
pub fn add(db: &str, path: &str, version: &str) -> std::io::Result<()> {
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
        .expect("could not create record in db");
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
    println!("listing versions");
    Ok(())
}

fn version_by_id(db: &Connection, id: i32) -> Result<String> {
    let versions: Vec<Version> = Version::select(&db, "id, name", |row| {
        Ok(Version {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    let version: Version = versions
        .into_iter()
        .filter(|v| v.id == id)
        .nth(0)
        .expect("version not found");
    Ok(version.name)
}

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
            let version_name = version_by_id(&db, config.version_id)?;
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
    let configs: Vec<Config> = Config::select(&db, "id, version_id, path, data", |row| {
        let data: String = row.get(3)?;
        Ok(Config {
            id: row.get(0)?,
            version_id: row.get(1)?,
            path: row.get(2)?,
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
    let db = get_db(DEFAULT_DB_LOCATION);

    Config::table(&db)?;
    let test_config = Config {
        id: 1,
        version_id: 1,
        path: "/tmp/test".to_string(),
        data: vec!["first line".to_string(), "second line".to_string()],
    };
    test_config.create(&db)?;
    let configs: Vec<Config> = Config::select(&db, "id, version_id, path, data", |row| {
        let data: String = row.get(3)?;
        Ok(Config {
            id: row.get(0)?,
            version_id: row.get(1)?,
            path: row.get(2)?,
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
    let db = get_db(DEFAULT_DB_LOCATION);

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
