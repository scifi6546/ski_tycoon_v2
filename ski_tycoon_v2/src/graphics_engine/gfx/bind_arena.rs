use generational_arena::{Arena, Index};
#[derive(Clone)]
pub struct BindArenaIndex {
    index: Index,
}
#[allow(dead_code)]
pub struct BindArena<T> {
    arena: Arena<T>,
    currently_bound: Option<Index>,
}
#[allow(dead_code)]
impl<T> BindArena<T> {
    pub fn insert(&mut self, data: T) -> BindArenaIndex {
        BindArenaIndex {
            index: self.arena.insert(data),
        }
    }
    pub fn get(&self, idx: BindArenaIndex) -> Option<&T> {
        self.arena.get(idx.index)
    }
    pub fn get_mut(&mut self, idx: BindArenaIndex) -> Option<&mut T> {
        self.arena.get_mut(idx.index)
    }
    pub fn get_bound(&self) -> Option<&T> {
        if let Some(idx) = self.currently_bound {
            self.arena.get(idx)
        } else {
            None
        }
    }
    pub fn bind(&mut self, idx: BindArenaIndex) {
        self.currently_bound = Some(idx.index);
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
