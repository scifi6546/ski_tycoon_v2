use generational_arena::{Arena, Index};
pub struct BindArena<T> {
    arena: Arena<T>,
    currently_bound: Option<Index>,
}
impl<T> BindArena<T> {
    pub fn insert(&mut self, data: T) -> Index {
        self.arena.insert(data)
    }
    pub fn get(&self, idx: Index) -> Option<&T> {
        self.arena.get(idx)
    }
    pub fn get_bound(&self) -> Option<&T> {
        if let Some(idx) = self.currently_bound {
            self.arena.get(idx)
        } else {
            None
        }
    }
    pub fn bind(&mut self, idx: Index) {
        self.currently_bound = Some(idx);
    }
}
impl<T> Default for BindArena<T> {
    fn default() -> Self {
        Self {
            arena: Arena::new(),
            currently_bound: None,
        }
    }
}
