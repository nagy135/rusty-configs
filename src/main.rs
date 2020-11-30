use rusqlite::{ Connection, NO_PARAMS };
fn main() {
    let conn = Connection::open("myfile.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
    )", NO_PARAMS).unwrap();
    let name: String = "Steve Example".to_string();
    let email: String = "steve@example.org".to_string();
    conn.execute("INSERT INTO person (name, email) VALUES (?1, ?2)",
    &[&name, &email]).unwrap();
}
