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
    pub fn get_or_create(&mut self, key: &str, ctor: fn() -> T) -> &T {
        if self.data.contains_key(key) {
            self.data.get(key).unwrap()
        } else {
            self.data.insert(key.to_string(), ctor());
            self.data.get(key).unwrap()
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    fn zero() -> u32 {
        0
    }
    fn one() -> u32 {
        1
    }
    #[test]
    fn manage() {
        let mut manager = AssetManager::default();
        assert_eq!(manager.get_or_create("zero", zero), &0);
        assert_eq!(manager.get_or_create("zero", one), &0);
        assert_eq!(manager.get_or_create("one", one), &1);
    }
}
