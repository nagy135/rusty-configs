use rusqlite::{params, Connection, Result, NO_PARAMS};

mod entities;
use entities::{Config, Version, Entity};

fn main() -> Result<()> {
    Ok(())
}


#[test]
fn db_test_config() -> Result<()> {
    // let conn = Connection::open("myfile.db").unwrap();
    let conn = Connection::open_in_memory()?;

    conn.execute(
        &Config::create_table(),
        NO_PARAMS
    )?;

    let test_config = Config {
        id: 1,
        path: "/tmp/test".to_string(),
        data: vec!["first line".to_string(), "second line".to_string()],
    };
    conn.execute(
        "INSERT INTO configs (path, data) VALUES (?1, ?2)",
        params![test_config.path, test_config.data.join("\n")],
    )?;

    let mut stmt = conn.prepare("SELECT id, path, data FROM configs")?;
    let mut configs = stmt.query_map(NO_PARAMS, |row| {
        let data: String = row.get(2)?;
        Ok(Config {
            id: row.get(0)?,
            path: row.get(1)?,
            data: data.split('\n').into_iter().map(|x| x.to_string()).collect()
        })
    })?;

    let fetched_config: Config = configs.next().unwrap()?;
    assert_eq!(1, fetched_config.id);
    assert_eq!("/tmp/test", fetched_config.path);
    assert_eq!(vec!["first line".to_string(), "second line".to_string()], fetched_config.data);
    Ok(())
}

#[test]
fn db_test_version() -> Result<()> {
    // let conn = Connection::open("myfile.db").unwrap();
    let conn = Connection::open_in_memory()?;

    conn.execute(
        &Version::create_table(),
        NO_PARAMS
    )?;

    let test_version = Version {
        id: 1,
        name: "home".to_string(),
    };
    conn.execute(
        "INSERT INTO versions (name) VALUES (?1)",
        params![test_version.name],
    )?;

    let mut stmt = conn.prepare("SELECT id, name FROM versions")?;
    let mut versions = stmt.query_map(NO_PARAMS, |row| {
        Ok(Version {
            id: row.get(0)?,
            name: row.get(1)?
        })
    })?;

    let fetched_version: Version = versions.next().unwrap()?;
    assert_eq!(1, fetched_version.id);
    assert_eq!("home", fetched_version.name);
    Ok(())
}
