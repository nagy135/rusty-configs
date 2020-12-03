use rusqlite::{Connection, Result, Row, NO_PARAMS};

#[derive(Debug)]
pub struct Config {
    pub id: i32,
    pub path: String,
    pub data: Vec<String>,
}

#[derive(Debug)]
pub struct Version {
    pub id: i32,
    pub name: String,
}

pub trait Entity<'a> {
    fn table_name() -> &'static str;
    fn types() -> &'static str;

    // These two has to have same values
    fn values(&self) -> String;
    fn columns() -> &'static str;

    fn create(&self, db: &'a Connection) -> Result<()> {
        db.execute(
            &format!(
                "{} {} {} {} ({})",
                "INSERT INTO",
                Self::table_name(),
                Self::columns(),
                "VALUES",
                Self::values(&self)
            ),
            NO_PARAMS,
        )?;
        Ok(())
    }

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

impl<'a> Entity<'a> for Config {
    fn table_name() -> &'static str {
        "configs"
    }
    fn types() -> &'static str {
        "(id PRIMARY KEY,
        path NOT NULL,
        data NOT NULL)"
    }
    fn columns() -> &'static str {
        "(id, path, data)"
    }
    fn values(&self) -> String {
        format!("{}, '{}', '{}'", self.id, self.path, self.data.join("\n"))
    }
}

impl<'a> Entity<'a> for Version {
    fn table_name() -> &'static str {
        "versions"
    }
    fn columns() -> &'static str {
        "(id, name)"
    }
    fn types() -> &'static str {
        "(id PRIMARY KEY,
        name NOT NULL)"
    }
    fn values(&self) -> String {
        format!("{}, '{}'", self.id, self.name)
    }
}
