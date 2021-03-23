use super::prelude::{
    build_skiier, insert_lift, insert_terrain, AssetManager, GraphLayer, GraphWeight, Grid,
    GridNode, Model, RenderingContext, RuntimeModel, ShaderBind, Transform,
};
use egui::CtxRef;
use legion::World;
use log::info;
use nalgebra::{Vector2, Vector3};
mod pgm_parser;
pub struct TerrainLibrary {
    entries: Vec<Scenario>,
}
impl Default for TerrainLibrary {
    fn default() -> Self {
        Self {
            entries: vec![
                Scenario {
                    name: "Cone World".to_string(),
                    terrain_ctor: Box::new(|| {
                        Terrain::new_cone(
                            Vector2::new(20, 20),
                            Vector2::new(10.0, 10.0),
                            10.0,
                            -1.0,
                        )
                    }),
                    skiier_spawn: (0..10)
                        .map(|x| (0..10).map(move |y| Vector2::new(x.clone(), y.clone())))
                        .flatten()
                        .collect(),
                    lift_positions: vec![LiftPosition {
                        start: Vector2::new(0, 0),
                        end: Vector2::new(3, 3),
                    }],
                },
                Scenario {
                    name: "Small Cone World".to_string(),
                    terrain_ctor: Box::new(|| {
                        Terrain::new_cone(Vector2::new(5, 5), Vector2::new(10.0, 10.0), 10.0, 1.0)
                    }),
                    skiier_spawn: vec![],
                    lift_positions: vec![],
                },
                Scenario {
                    name: "Toture Test".to_string(),
                    terrain_ctor: Box::new(|| {
                        Terrain::new_cone(
                            Vector2::new(100, 100),
                            Vector2::new(50.0, 50.0),
                            50.0,
                            -1.0,
                        )
                    }),
                    skiier_spawn: (0..100)
                        .map(|x| (0..100).map(move |y| Vector2::new(x, y)))
                        .flatten()
                        .collect(),
                    lift_positions: vec![LiftPosition {
                        start: Vector2::new(0, 0),
                        end: Vector2::new(50, 50),
                    }],
                },
                Scenario {
                    name: "PGM File".to_string(),
                    terrain_ctor: Box::new(|| {
                        Terrain::from_pgm(include_bytes!("heightmaps/output.pgm").to_vec(), 0.01)
                            .unwrap()
                    }),
                    skiier_spawn: (0..5)
                        .map(|x| (0..1).map(move |y| Vector2::new(x.clone(), y.clone())))
                        .flatten()
                        .collect(),
                    lift_positions: vec![LiftPosition {
                        start: Vector2::new(0, 0),
                        end: Vector2::new(7, 3),
                    }],
                },
                Scenario {
                    name: "PGM File No Skiiers".to_string(),
                    terrain_ctor: Box::new(|| {
                        Terrain::from_pgm(include_bytes!("heightmaps/output.pgm").to_vec(), 0.01)
                            .unwrap()
                    }),
                    skiier_spawn: vec![],
                    lift_positions: vec![LiftPosition {
                        start: Vector2::new(0, 0),
                        end: Vector2::new(7, 3),
                    }],
                },
                Scenario {
                    name: "Volcano".to_string(),
                    terrain_ctor: Box::new(|| {
                        Terrain::from_pgm(include_bytes!("heightmaps/cone.pgm").to_vec(), 0.001)
                            .unwrap()
                    }),
                    skiier_spawn: (0..20)
                        .map(|x| (0..20).map(move |y| Vector2::new(x.clone(), y.clone())))
                        .flatten()
                        .collect(),
                    lift_positions: vec![LiftPosition {
                        start: Vector2::new(0, 0),
                        end: Vector2::new(50, 50),
                    }],
                },
            ],
        }
    }
}
pub struct LiftPosition {
    start: Vector2<i64>,
    end: Vector2<i64>,
}
pub struct Scenario {
    pub name: String,
    pub terrain_ctor: Box<dyn Fn() -> Terrain>,
    pub skiier_spawn: Vec<Vector2<i64>>,
    pub lift_positions: Vec<LiftPosition>,
}
impl Scenario {
    pub fn build_scenario(
        &self,
        world: &mut World,
        graphics: &mut RenderingContext,
        asset_manager: &mut AssetManager<RuntimeModel>,
        bound_shader: &ShaderBind,
    ) {
        world.clear();
        info!("building scene: {}", self.name);

        insert_terrain(
            (self.terrain_ctor)(),
            world,
            graphics,
            asset_manager,
            bound_shader.get_bind(),
        )
        .expect("failed to insert terrain");
        for l in self.lift_positions.iter() {
            insert_lift(world, graphics, asset_manager, bound_shader, l.start, l.end)
                .expect("failed to build skiier");
        }
        for s in self.skiier_spawn.iter() {
            build_skiier(world, graphics, asset_manager, bound_shader, s.clone())
                .expect("failed to build skiier");
        }
    }
}
impl TerrainLibrary {
    pub fn draw_gui(
        &self,
        world: &mut World,
        context: &mut CtxRef,
        graphics: &mut RenderingContext,
        asset_manager: &mut AssetManager<RuntimeModel>,
        bound_shader: &ShaderBind,
    ) {
        egui::Window::new("Scenarios").show(context, |ui| {
            for t in self.entries.iter() {
                ui.label(format!("{}", t.name));
                if ui.button("").clicked {
                    t.build_scenario(world, graphics, asset_manager, bound_shader);
                }
            }
        });
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Snow,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub height: f32,
    pub tile_type: TileType,
}
#[derive(Clone, Debug, PartialEq)]
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

    pub fn from_pgm(data: Vec<u8>, scaling: f32) -> Option<Self> {
        if let Some(s) = String::from_utf8(data).ok() {
            if let Some(t) = pgm_parser::terrain_from_pgm(s, TileType::Snow, scaling).ok() {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn from_tiles(tiles: Vec<Tile>, dimensions: Vector2<usize>) -> Self {
        Self { tiles, dimensions }
    }

    pub fn model(&self) -> Model {
        let heights = self.tiles.iter().map(|t| t.height).collect();
        Model::from_heights(heights, self.dimensions, Transform::default())
    }
    pub fn get_transform_rounded(&self, coordinate: &Vector2<f32>) -> Vector3<f32> {
        let x: i64 = unsafe { coordinate.x.to_int_unchecked() };
        let y: i64 = unsafe { coordinate.y.to_int_unchecked() };
        let convert_dimensions = |i: i64, i_dimensions: i64| {
            if i >= i_dimensions {
                i_dimensions - 1
            } else if i < 0 {
                0
            } else {
                i
            }
        };
        self.get_transform(&Vector2::new(
            convert_dimensions(x, self.dimensions.x as i64),
            convert_dimensions(y, self.dimensions.y as i64),
        ))
        .unwrap()
    }
    pub fn get_transform(&self, coordinate: &Vector2<i64>) -> Option<Vector3<f32>> {
        let pos = coordinate.x as usize * self.dimensions.y + coordinate.y as usize;
        if pos < self.tiles.len() {
            Some(Vector3::new(
                coordinate.x as f32,
                self.tiles[pos].height,
                coordinate.y as f32,
            ))
        } else {
            None
        }
    }
    fn get_weight(&self, start: Vector2<i64>, end: Vector2<i64>) -> GraphWeight {
        if end.x >= self.dimensions.x as i64
            || end.x < 0
            || end.y >= self.dimensions.y as i64
            || end.y < 0
            || start.x >= self.dimensions.x as i64
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
                GraphWeight::Some((delta_height * 100.0).abs() as i32)
            } else {
                GraphWeight::Some((delta_height * 10.0).abs() as i32)
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
