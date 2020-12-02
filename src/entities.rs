use rusqlite::{params, Connection, Result, NO_PARAMS};

#[derive(Debug)]
pub struct Config {
    pub id: i32,
    pub path: String,
    pub data: Vec<String>
}

#[derive(Debug)]
pub struct Version {
    pub id: i32,
    pub name: String
}

pub trait Entity<'a> {
    fn table_name() -> &'static str;
    fn columns() -> &'static str;
    fn values(&self) -> String;
    fn types() -> &'static str;

    fn create(&self, db: &'a Connection) -> Result<()> {
        db.execute(
            &format!(
                "{} {} ({}) {} {}",
                "INSERT INTO",
                Self::table_name(),
                Self::columns(),
                "VALUES",
                Self::values(&self)
            ),
            NO_PARAMS
        )?;
        Ok(())
    }

    fn table(&self, db: &'a Connection) -> Result<()> {
        db.execute(
            &format!(
                "{} {} ({})",
                "CREATE TABLE IF NOT EXISTS",
                Self::table_name(),
                Self::types()
            ),
            NO_PARAMS
        )?;
        Ok(())
    }
}

impl<'a> Entity<'a> for Config {
    fn table_name() -> &'static str {
        "configs"
    }
    fn columns() -> &'static str {
        "path, data"
    }
    fn types() -> &'static str {
        "id PRIMARY KEY,
        path NOT NULL,
        data NOT NULL"
    }
    fn values(&self) -> String {
        format!(
            "{}, {}",
            self.path,
            self.data.join("\n")
        )
    }

}

impl<'a> Entity<'a> for Version {
    fn table_name() -> &'static str {
        "versions"
    }
    fn columns() -> &'static str {
        "name"
    }
    fn types() -> &'static str {
        "id PRIMARY KEY,
        name NOT NULL"
    }
    fn values(&self) -> String {
        self.name.clone()
    }

}
