use std::collections::HashMap;
pub struct AssetManager<T> {
    data: HashMap<String, T>,
}
impl<T> Default for AssetManager<T> {
    fn default() -> Self {
        AssetManager {
            data: HashMap::new(),
        }
    }
}
impl<T> AssetManager<T> {
    pub fn get_or_create(&mut self, key: &str, data: T) -> &T {
        if self.data.contains_key(key) {
            self.data.get(key).unwrap()
        } else {
            self.data.insert(key.to_string(), data);
            self.data.get(key).unwrap()
        }
    }
    /// Sets data at key point overwrites if it is already present
    pub fn overwrite(&mut self, key: &str, data: T) -> &T {
        self.data.insert(key.to_string(), data);
        //must have data at key because it as just inserted
        self.data.get(key).unwrap()
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        self.data.get(key)
    }
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn manage() {
        let mut manager = AssetManager::default();
        assert_eq!(manager.contains("zero"), false);
        assert_eq!(manager.get_or_create("zero", 0), &0);
        assert_eq!(manager.get_or_create("zero", 1), &0);
        assert_eq!(manager.get("zero"), Some(&0));
        assert_eq!(manager.get("one"), None);
        assert_eq!(manager.contains("zero"), true);
        assert_eq!(manager.get_or_create("one", 1), &1);
        assert_eq!(manager.overwrite("zero", 1), &1);
        assert_eq!(manager.overwrite("zero", 0), &0);
        assert_eq!(manager.overwrite("three", 3), &3);
    }
}
