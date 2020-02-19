use std::collections::HashMap;
pub struct KvStore{
    dict: HashMap<String, String>,
}

impl KvStore {
    /// Creates a `KvStore`.
    pub fn new() -> KvStore {
        //unimplemented!()
        KvStore{
            dict: HashMap::new()
        }
    }

    /// Sets the value of a string key to a string.
    ///
    /// If the key already exists, the previous value will be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        //unimplemented!()
        self.dict.insert(key, value);
    }

    /// Gets the string value of a given string key.
    ///
    /// Returns `None` if the given key does not exist.
    pub fn get(&self, key: String) -> Option<String> {
        //unimplemented!()
        self.dict.get(&key).cloned()
    }

    /// Removes a given key.
    pub fn remove(&mut self, key: String) {
        //unimplemented!()
        self.dict.remove(&key);
    }
}
