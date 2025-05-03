use std::collections::HashMap;

/// InMemoryDb is a simple in-memory key-value store.
pub struct InMemoryDb {
    store: HashMap<String, String>,
}

impl InMemoryDb {
    /// Create a new, empty database.
    pub fn new() -> Self {
        InMemoryDb {
            store: HashMap::new(),
        }
    }

    /// Set the value for a key.
    pub fn set(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    /// Get the value for a key, if it exists.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    /// Delete a key from the database. Returns true if the key was present.
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_delete() {
        let mut db = InMemoryDb::new();
        db.set("foo".to_string(), "bar".to_string());
        assert_eq!(db.get("foo"), Some(&"bar".to_string()));
        assert_eq!(db.delete("foo"), true);
        assert_eq!(db.get("foo"), None);
        assert_eq!(db.delete("foo"), false);
    }
}
