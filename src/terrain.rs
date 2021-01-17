use super::prelude::{GraphLayer, GraphWeight, Grid, GridNode, Model, Transform};
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
    fn get_weight(&self, start: Vector2<i64>, end: Vector2<i64>) -> GraphWeight {
        if end.x >= self.dimensions.x as i64
            || end.x < 0
            || end.y >= self.dimensions.y as i64
            || end.y < 0
        {
            GraphWeight::Infinity
        } else if start.x >= self.dimensions.x as i64
            || start.x < 0
            || start.y >= self.dimensions.y as i64
            || start.y < 0
        {
            GraphWeight::Infinity
        } else {
            let start_tile = &self.tiles[start.x as usize * self.dimensions.y + start.y as usize];
            let end_tile = &self.tiles[end.x as usize * self.dimensions.y + end.y as usize];
            let delta_height = start_tile.height - end_tile.height;
            if delta_height as i32 >= 0 {
                GraphWeight::Some(delta_height as i32 * 10 as i32)
            } else {
                GraphWeight::Some(delta_height as i32)
            }
        }
    }
    pub fn build_graph(&self) -> GraphLayer {
        let mut data = vec![];
        data.reserve(self.dimensions.x * self.dimensions.y);
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let x_plus = self.get_weight(
                    Vector2::new(x as i64, y as i64),
                    Vector2::new(x as i64 + 1, y as i64),
                );
                let x_minus = self.get_weight(
                    Vector2::new(x as i64, y as i64),
                    Vector2::new(x as i64 - 1, y as i64),
                );
                let z_plus = self.get_weight(
                    Vector2::new(x as i64, y as i64),
                    Vector2::new(x as i64, y as i64 + 1),
                );
                let z_minus = self.get_weight(
                    Vector2::new(x as i64, y as i64),
                    Vector2::new(x as i64, y as i64 + 1),
                );
                data.push(GridNode {
                    x_plus,
                    x_minus,
                    z_plus,
                    z_minus,
                });
            }
        }
        let grid = Grid::from_vec(data, self.dimensions);
        GraphLayer::Grid { grid }
    }
}
