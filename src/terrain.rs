use super::prelude::{Model, Transform};
use nalgebra::Vector2;
#[derive(Clone, Debug, PartialEq)]
enum TileType {
    Snow,
}
#[derive(Clone, Debug, PartialEq)]
struct Tile {
    height: f32,
    tile_type: TileType,
}
pub struct Terrain {
    tiles: Vec<Tile>,
    dimensions: Vector2<usize>,
}
impl Terrain {
    /// Builds cone terrain with centar at center and slope of `slope`
    pub fn new_cone(
        dimensions: Vector2<usize>,
        center: Vector2<f32>,
        center_height: f32,
        slope: f32,
    ) -> Self {
        let mut tiles = vec![];
        tiles.reserve(dimensions.x * dimensions.y);
        for x in 0..dimensions.x {
            for y in 0..dimensions.y {
                let radius = ((x as f32 - center.x).powi(2) + (y as f32 - center.y).powi(2)).sqrt();
                let height = center_height + radius * slope;
                tiles.push(Tile {
                    height,
                    tile_type: TileType::Snow,
                });
            }
        }
        Self { tiles, dimensions }
    }
    pub fn model(&self) -> Model {
        let heights = self.tiles.iter().map(|t| t.height).collect();
        Model::from_heights(heights, self.dimensions, Transform::default())
    }
}
