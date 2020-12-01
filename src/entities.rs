
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

pub trait Entity {
    fn create_table() -> String;
    fn table_name() -> &'static str;
}

impl Entity for Config {
    fn table_name() -> &'static str {
        "configs"
    }

    fn create_table() -> String {
        format!(
            "{} {} {}",
            "CREATE TABLE IF NOT EXISTS", Self::table_name() , " (
                  id              INTEGER PRIMARY KEY,
                  path            TEXT NOT NULL,
                  data            TEXT NOT NULL
                  )"
        )
    }
}

impl Entity for Version {
    fn table_name() -> &'static str {
        "versions"
    }

    fn create_table() -> String {
        format!(
            "{} {} {}",
            "CREATE TABLE IF NOT EXISTS", Self::table_name() , " (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )"
        )
    }
}
