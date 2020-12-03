#[allow(unused_imports)]
use rusqlite::{params, Connection, Result, NO_PARAMS};

mod entities;

#[allow(unused_imports)]
use entities::{Config, Version, Entity};

fn main() -> Result<()> {
    Ok(())
}

#[test]
fn config_entity() -> Result<()> {
    // let db = Connection::open("myfile.db").unwrap();
    let db = Connection::open_in_memory()?;
    let test_config = Config {
        id: 1,
        path: "/tmp/test".to_string(),
        data: vec!["first line".to_string(), "second line".to_string()],
    };
    test_config.table(&db)?;
    test_config.create(&db)?;
    let configs: Vec<Config> = test_config.select(&db, "id, path, data", |row| {
        let data: String = row.get(2)?;
        Ok(Config {
            id: row.get(0)?,
            path: row.get(1)?,
            data: data.split('\n').into_iter().map(|x| x.to_string()).collect()
        })
    })?;
    assert_eq!(1, configs[0].id);
    assert_eq!("/tmp/test", configs[0].path);
    assert_eq!(vec!["first line".to_string(), "second line".to_string()], configs[0].data);
    Ok(())
}

#[test]
fn version_entity() -> Result<()> {
    // let db = Connection::open("myfile.db").unwrap();
    let db = Connection::open_in_memory()?;
    let test_version = Version {
        id: 1,
        name: "home".to_string()
    };
    test_version.table(&db)?;
    test_version.create(&db)?;
    let versions: Vec<Version> = test_version.select(&db, "id, name", |row| {
        Ok(Version {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;
    assert_eq!(1, versions[0].id);
    assert_eq!("home", versions[0].name);
    Ok(())
}
