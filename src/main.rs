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
    let db = Connection::open("myfile.db").unwrap();
    // let db = Connection::open_in_memory()?;
    let test_config = Config {
        id: 1,
        path: "/tmp/test".to_string(),
        data: vec!["first line".to_string(), "second line".to_string()],
    };
    test_config.table(&db)?;
    test_config.create(&db)?;
    let configs: Vec<Config> = test_config.select(&db, "id, path, data", |row| {
        let data: String = row.get(2)?;
        let lol: i64 = row.get(0)?;
        println!("hahahahaha{}", data);
        Ok(Config {
            id: 1,
            // path: "/tmp/test".to_string(),
            // data: vec!["first line".to_string(), "second line".to_string()],
            // id: row.get(0)?,
            path: row.get(1)?,
            data: data.split('\n').into_iter().map(|x| x.to_string()).collect()
        })
    })?;
    assert_eq!(1, configs[0].id);
    assert_eq!("/tmp/test", configs[0].path);
    assert_eq!(vec!["first line".to_string(), "second line".to_string()], configs[0].data);
    Ok(())

    // let mut stmt = db.prepare("SELECT id, path, data FROM configs")?;
    // let mut configs = stmt.query_map(NO_PARAMS, |row| {
    //     let data: String = row.get(2)?;
    //     Ok(Config {
    //         id: row.get(0)?,
    //         path: row.get(1)?,
    //         data: data.split('\n').into_iter().map(|x| x.to_string()).collect()
    //     })
    // })?;

    // let fetched_config: Config = configs.next().unwrap()?;
    // assert_eq!(1, fetched_config.id);
    // assert_eq!("/tmp/test", fetched_config.path);
    // assert_eq!(vec!["first line".to_string(), "second line".to_string()], fetched_config.data);
}


// #[test]
// fn db_test_version() -> Result<()> {
//     // let db = Connection::open("myfile.db").unwrap();
//     let db = Connection::open_in_memory()?;

//     db.execute(
//         &Version::table(),
//         NO_PARAMS
//     )?;

//     let test_version = Version {
//         id: 1,
//         name: "home".to_string(),
//     };
//     db.execute(
//         "INSERT INTO versions (name) VALUES (?1)",
//         params![test_version.name],
//     )?;

//     let mut stmt = db.prepare("SELECT id, name FROM versions")?;
//     let mut versions = stmt.query_map(NO_PARAMS, |row| {
//         Ok(Version {
//             id: row.get(0)?,
//             name: row.get(1)?
//         })
//     })?;

//     let fetched_version: Version = versions.next().unwrap()?;
//     assert_eq!(1, fetched_version.id);
//     assert_eq!("home", fetched_version.name);
//     Ok(())
// }
