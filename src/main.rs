use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();
    println!("The key is: {} and the value is: {}", key, value);

    // let contents = format!("{}\t{}\n", key, value);
    // std::fs::write("kv.db", contents).unwrap();

    let mut database = Database::new().expect("Database::new crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);

    database.flush().unwrap();
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        // Read the kv.db file
        let mut map = HashMap::new();
        // let contents = match std::fs::read_to_string("kv.db") {
        //     Ok(c) => c,
        //     Err(error) => {
        //         return Err(error);
        //     }
        // };
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        // let mut contents = String::new();
        // for (key, value) in &self.map {
        //     // let pair = format!("{}\t{}\n", key, value);
        //     // contents.push_str(&pair)
        //     contents.push_str(key);
        //     contents.push('\t');
        //     contents.push_str(value);
        //     contents.push('\n');
        // }
        // std::fs::write("kv.db", contents)
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // let mut contents = String::new();
        // for (key, value) in &self.map {
        //     // let pair = format!("{}\t{}\n", key, value);
        //     // contents.push_str(&pair)
        //     contents.push_str(key);
        //     contents.push('\t');
        //     contents.push_str(value);
        //     contents.push('\n');
        // }
        // let _ = std::fs::write("kv.db", contents);
        if !self.flush {
            let _ = do_flush(self);
        }
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        // let pair = format!("{}\t{}\n", key, value);
        // contents.push_str(&pair)
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}
