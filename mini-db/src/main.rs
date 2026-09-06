use std::collections::HashMap;

struct MiniDb {
    store: HashMap<String, String>,
}

impl MiniDb {
    fn new() -> Self {
        MiniDb {
            store: HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.store.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
}

fn main() {
    let mut db = MiniDb::new();

    db.insert("name", "Dave");
    db.insert("city", "Hudson");

    println!("name = {:?}", db.get("name"));
    println!("city = {:?}", db.get("city"));

    db.delete("city");
    println!("city after delete = {:?}", db.get("city"));
}
