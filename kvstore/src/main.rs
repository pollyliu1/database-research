use rocksdb::{DB, Options};

fn init_db(path: &str) -> DB {
    let mut options = Options::default();
    options.create_if_missing(true);
    DB::open(&options, path).expect("Failed to open RocksDB")
}

fn put(db: &DB, key: &str, value: &str) {
    db.put(key.as_bytes(), value.as_bytes()).expect("Failed to write to RocksDB");
}

fn get(db: &DB, key: &str) -> Option<String> {
    match db.get(key.as_bytes()) {
        Ok(Some(value)) => Some(String::from_utf8(value).expect("Failed to read value")),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Error reading from RocksDB: {:?}", e);
            None
        }
    }
}

fn delete(db: &DB, key: &str) {
    db.delete(key.as_bytes()).expect("Failed to delete from RocksDB");
}

fn main() {
    let db_path = "kvstore_db";
    let db = init_db(db_path);

    // PUT
    put(&db, "key1", "value2");
    println!("Inserted key1 with value2");

    // GET
    match get(&db, "key1") {
        Some(value) => println!("Retrieved key1: {}", value),
        None => println!("Key not found"),
    }

    // DELETE
    delete(&db, "key1");
    println!("Deleted key1");
}
