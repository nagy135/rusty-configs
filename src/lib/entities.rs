use rusqlite::{Connection, Result, Row, NO_PARAMS};

/// Entity representing config stored in db
#[derive(Debug)]
pub struct Config {
    pub id: i32,
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

    /// values during create, has to have the same number of items
    /// separated by comma as self::columns
    fn values(&self) -> String;

    /// creates db instance of entity
    fn create(&self, db: &'a Connection) -> Result<()> {
        db.execute(
            &format!(
                "INSERT INTO {} {} VALUES ({})",
                Self::table_name(),
                Self::columns(),
                Self::values(&self)
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// delete entity from db
    fn delete(db: &'a Connection, parameter: &str, value: &str) -> Result<()> {
        println!(
            "DELETE FROM {} WHERE {}={}",
            Self::table_name(),
            parameter,
            value
        );
        db.execute(
            &format!(
                "DELETE FROM {} WHERE {}={}",
                Self::table_name(),
                parameter,
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
                "{} {} {}",
                "CREATE TABLE IF NOT EXISTS",
                Self::table_name(),
                Self::types()
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

    /// fetches fields of entity passed in query and returns Vec<Self>
    fn select<F>(db: &'a Connection, query: &str, f: F) -> Result<Vec<Self>>
    where
        F: FnMut(&Row<'_>) -> Result<Self>,
        Self: Sized,
    {
        let mut stmt = db.prepare(&format!(
            "{} {} {} {}",
            "SELECT",
            query,
            "FROM",
            Self::table_name()
        ))?;
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
        data TEXT NOT NULL)"
    }
    fn columns() -> &'static str {
        "(id, path, data)"
    }
    fn values(&self) -> String {
        format!("{}, '{}', '{}'\n", self.id, self.path, self.data.join("\n"))
    }
}

/// implementation of Entity trait for Version
impl<'a> Entity<'a> for Version {
    fn table_name() -> &'static str {
        "versions"
    }
    fn columns() -> &'static str {
        "(id, name)"
    }
    fn types() -> &'static str {
        "(id PRIMARY KEY,
        name TEXT NOT NULL)"
    }
    fn values(&self) -> String {
        format!("{}, '{}'", self.id, self.name)
    }
}
