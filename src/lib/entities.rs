extern crate base64;

use base64::{decode, encode};
use rusqlite::{Connection, Result, Row, NO_PARAMS};

/// Entity representing config stored in db
#[derive(Debug)]
pub struct Config {
    pub id: i32,
    pub version_id: i32,
    pub path: String,
    pub data: Vec<String>,
}

/// Entity representing version of configs
#[derive(Debug)]
pub struct Version {
    pub id: i32,
    pub name: String,
}

pub trait Entity<'a> {
    /// name of the table (statically defined)
    fn table_name() -> &'static str;
    /// types of data fields (statically defined)
    fn types() -> &'static str;
    /// columns representing fields of entity, for create (statically defined)
    fn columns() -> &'static str;

    /// returns vector of all entities
    fn all(db: &Connection) -> Result<Vec<Self>>
    where
        Self: Sized;

    /// returns vector of all entities (general part)
    fn _all<F>(db: &Connection, f: F) -> Result<Vec<Self>>
    where
        Self: Sized,
        F: FnMut(&Row<'_>) -> Result<Self>,
    {
        Self::select(db, Self::columns(), f)
    }

    /// returns vectors of all entities matching condition
    fn select_where(db: &Connection, condition: &str) -> Result<Vec<Self>>
    where
        Self: Sized;

    /// fetches fields of entity passed in query matching where clause and returns Vec<Self>
    fn _select_where<F>(db: &Connection, condition: &str, f: F) -> Result<Vec<Self>>
    where
        F: FnMut(&Row<'_>) -> Result<Self>,
        Self: Sized,
    {
        let mut stmt = db.prepare(&format!(
            "SELECT {} FROM {} WHERE {}",
            Self::columns(),
            Self::table_name(),
            condition
        ))?;
        let results = stmt.query_map(NO_PARAMS, f)?;

        let rows: Vec<Self> = results.into_iter().map(|e| e.unwrap()).collect();
        Ok(rows)
    }

    /// return entity instance by its id (general part)
    fn find(db: &Connection, id: i32) -> Result<Self>
    where
        Self: Sized;
    fn _find<F>(db: &Connection, id: i32, f: F) -> Result<Self>
    where
        Self: Sized,
        F: FnMut(&Row<'_>) -> Result<Self>,
    {
        let mut stmt = db.prepare(&format!(
            "SELECT {} FROM {} WHERE id={}",
            Self::columns(),
            Self::table_name(),
            id
        ))?;
        stmt.query_row(NO_PARAMS, f)
    }

    fn next_id(db: &'a Connection) -> Result<i32> {
        let mut stmt = db.prepare(&format!(
            "SELECT id FROM {} ORDER BY id DESC LIMIT 0, 1",
            Self::table_name()
        ))?;
        let highest_id: Option<i32> = stmt
            .query_map(NO_PARAMS, |row| row.get(0))?
            .into_iter()
            .map(|e| e.unwrap())
            .nth(0);
        match highest_id {
            Some(id) => Ok(id + 1),
            None => Ok(1),
        }
    }
    /// values during create, has to have the same number of items
    /// separated by comma as self::columns
    fn values(&self) -> String;

    /// creates db instance of entity
    fn create(&self, db: &'a Connection) -> Result<()> {
        db.execute(
            &format!(
                "INSERT INTO {} ({}) VALUES ({})",
                Self::table_name(),
                Self::columns(),
                Self::values(&self)
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// delete entity from db
    fn delete(db: &'a Connection, parameter: &str, value: &str, operator: &str) -> Result<()> {
        db.execute(
            &format!(
                "DELETE FROM {} WHERE {}{}{}",
                Self::table_name(),
                parameter,
                operator,
                value
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// update entity in db
    fn update(db: &'a Connection, id: i32, updated_column: &str, new_value: &str) -> Result<()> {
        db.execute(
            &format!(
                "UPDATE {} SET {}='{}' WHERE id={}",
                Self::table_name(),
                updated_column,
                new_value,
                id
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// creates table in the database according to table_name and data types
    fn table(db: &'a Connection) -> Result<()> {
        db.execute(
            &format!(
                "CREATE TABLE IF NOT EXISTS {} {}",
                Self::table_name(),
                Self::types()
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// fetches fields of entity passed in query and returns Vec<Self>
    fn select<F>(db: &Connection, query: &str, f: F) -> Result<Vec<Self>>
    where
        F: FnMut(&Row<'_>) -> Result<Self>,
        Self: Sized,
    {
        let mut stmt = db.prepare(&format!("SELECT {} FROM {}", query, Self::table_name()))?;
        let results = stmt.query_map(NO_PARAMS, f)?;

        let rows: Vec<Self> = results.into_iter().map(|e| e.unwrap()).collect();
        Ok(rows)
    }
}

/// implementation of Entity trait for Config
impl<'a> Entity<'a> for Config {
    fn table_name() -> &'static str {
        "configs"
    }
    fn types() -> &'static str {
        "(id PRIMARY KEY,
        path TEXT NOT NULL,
        data TEXT NOT NULL,
        version_id INTEGER NOT NULL,
        FOREIGN KEY (version_id) REFERENCES versions(id)
        )"
    }
    fn columns() -> &'static str {
        "id, path, data, version_id"
    }

    fn all(db: &Connection) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        Self::_all(db, |row| {
            let data: String = row.get(2)?;
            Ok(Config {
                id: row.get(0)?,
                path: row.get(1)?,
                data: decode_lines(data)
                    .split('\n')
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                version_id: row.get(3)?,
            })
        })
    }
    /// returns vector of entities matching query (use exact sql query here)
    fn select_where(db: &Connection, condition: &str) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        Self::_select_where(db, condition, |row| {
            let data: String = row.get(2)?;
            Ok(Config {
                id: row.get(0)?,
                path: row.get(1)?,
                data: decode_lines(data)
                    .split('\n')
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                version_id: row.get(3)?,
            })
        })
    }
    /// return entity instance by its id
    fn find(db: &Connection, id: i32) -> Result<Self>
    where
        Self: Sized,
    {
        Self::_find(db, id, |row| {
            let data: String = row.get(2)?;
            Ok(Config {
                id: row.get(0)?,
                path: row.get(1)?,
                data: decode_lines(data)
                    .split('\n')
                    .into_iter()
                    .map(|x| x.to_string())
                    .collect(),
                version_id: row.get(3)?,
            })
        })
    }

    fn values(&self) -> String {
        format!(
            "{}, '{}', '{}', {}",
            self.id,
            self.path,
            encode(self.data.join("\n")),
            self.version_id
        )
    }
}

/// implementation of Entity trait for Version
impl<'a> Entity<'a> for Version {
    fn table_name() -> &'static str {
        "versions"
    }
    fn columns() -> &'static str {
        "id, name"
    }
    fn types() -> &'static str {
        "(id PRIMARY KEY,
        name TEXT NOT NULL)"
    }

    fn all(db: &Connection) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        Self::_all(db, |row| {
            Ok(Version {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
    }

    /// returns vector of entities matching query (use exact sql query here)
    fn select_where(db: &Connection, condition: &str) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        Self::_select_where(db, condition, |row| {
            Ok(Version {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
    }

    /// return entity instance by its id
    fn find(db: &Connection, id: i32) -> Result<Self>
    where
        Self: Sized,
    {
        Self::_find(db, id, |row| {
            Ok(Version {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
    }
    fn values(&self) -> String {
        format!("{}, '{}'", self.id, self.name)
    }
}

fn decode_lines(encoded_lines: String) -> String {
    let decoded_lines = decode(encoded_lines).expect("base64 decode failed");
    let string_lines = std::str::from_utf8(&decoded_lines).expect("utf8 -> str conversion failed");
    string_lines.to_string()
}
