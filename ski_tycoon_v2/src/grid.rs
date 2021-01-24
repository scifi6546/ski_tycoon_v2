use nalgebra::Vector2;
use std::vec::Vec;
/// Grid used for terrain and other features
/// ```
/// # use nalgebra::Vector2;
/// # use ski_tycoon_v2::prelude::Grid;
/// let g = Grid::from_vec(vec![0u8],Vector2::new(1,1));
/// assert_eq!(g[Vector2::new(0,0)],0);
/// ```
#[derive(Clone, Debug)]
pub struct Grid<T> {
    data: Vec<T>,
    dimensions: Vector2<usize>,
}
impl<T> Grid<T> {
    /// Gets data from a vec. panics if dimensions do not mattch length of data
    pub fn from_vec(data: Vec<T>, dimensions: Vector2<usize>) -> Self {
        assert_eq!(data.len(), dimensions.x * dimensions.y);
        Self { data, dimensions }
    }
    pub fn get(&self, index: Vector2<i64>) -> Option<&T> {
        if index.x < 0 || index.y < 0 {
            None
        } else {
            let i = index.x as usize * self.dimensions.y + index.y as usize;
            if i < self.data.len() {
                Some(&self.data[i])
            } else {
                None
            }
        }
    }
    pub fn width(&self) -> usize {
        self.dimensions.x
    }
    pub fn height(&self) -> usize {
        self.dimensions.y
    }
}
impl<T> std::ops::Index<Vector2<usize>> for Grid<T> {
    type Output = T;
    fn index(&self, index: Vector2<usize>) -> &Self::Output {
        let i = index.x * self.dimensions.y + index.y;
        assert!(i < self.data.len());
        &self.data[i]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn simple_create() {
        let g = Grid::from_vec(vec![0u8], Vector2::new(1, 1));
        assert_eq!(g[Vector2::new(0, 0)], 0);
        assert_eq!(g.get(Vector2::new(0, 1)), None);
    }
}
