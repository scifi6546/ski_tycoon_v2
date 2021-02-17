use std::collections::HashMap;
use std::ops::{Index, IndexMut};
///Used to toggle between bindable resources
pub struct Bindable<T> {
    data: HashMap<String, T>,
    currently_bound: Option<String>,
}
impl<T> Bindable<T> {
    pub fn insert(&mut self, key: &str, data: T) {
        self.data.insert(key.to_string(), data);
    }
    /// binds the key. panics if the key does not existg
    pub fn bind(&mut self, key: &str) {
        assert!(self.data.contains_key(key));
        self.currently_bound = Some(key.to_string())
    }
    /// gets the currently bound key. panics if nothing was bound yet
    pub fn get_bind(&self) -> &T {
        &self.data[self.currently_bound.as_ref().unwrap()]
    }
}
impl<T> Default for Bindable<T> {
    fn default() -> Self {
        Self {
            data: HashMap::new(),
            currently_bound: None,
        }
    }
}
impl<T> Index<&str> for Bindable<T> {
    type Output = T;
    fn index(&self, key: &str) -> &Self::Output {
        &self.data[key]
    }
}
impl<T> IndexMut<&str> for Bindable<T> {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.data.get_mut(index).unwrap()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_insert_and_bind() {
        let mut b = Bindable::default();
        b.insert("foo", 0u32);
        assert_eq!(b["foo"], 0u32);
    }
}
