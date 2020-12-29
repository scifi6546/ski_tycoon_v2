use super::prelude::Grid;
use nalgebra::Vector2;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GraphWeight {
    Some(u32),
    Infinity,
}
impl std::ops::Add for GraphWeight {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Self::Some(num) => match other {
                Self::Some(other_num) => Self::Some(num + other_num),
                Self::Infinity => Self::Infinity,
            },
            Self::Infinity => Self::Infinity,
        }
    }
}
impl std::cmp::PartialOrd for GraphWeight {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for GraphWeight {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Self::Infinity => match other {
                Self::Infinity => std::cmp::Ordering::Equal,
                Self::Some(_) => std::cmp::Ordering::Less,
            },
            Self::Some(s) => match other {
                Self::Infinity => std::cmp::Ordering::Greater,
                Self::Some(o) => s.cmp(o),
            },
        }
    }
}
pub struct GridNode {
    pub x_plus: GraphWeight,
    pub x_minus: GraphWeight,
    pub z_plus: GraphWeight,
    pub z_minus: GraphWeight,
}
//layer of graph system
pub enum GraphLayer {
    Grid { grid: Grid<GridNode> },
}
impl GraphLayer {
    pub fn get_children(&self, source: &Vector2<i64>) -> Vec<(Vector2<i64>, GraphWeight)> {
        match self {
            Self::Grid { grid } => {
                if let Some(node) = grid.get(source.clone()) {
                    vec![
                        (source + Vector2::new(1, 0), node.x_plus.clone()),
                        (source + Vector2::new(-1, 0), node.x_minus.clone()),
                        (source + Vector2::new(0, 1), node.z_plus.clone()),
                        (source + Vector2::new(0, -1), node.z_minus.clone()),
                    ]
                } else {
                    vec![]
                }
            }
        }
    }
    pub fn get(&self, source: Vector2<i64>, destination: Vector2<i64>) -> GraphWeight {
        match self {
            Self::Grid { grid } => {
                if let Some(node) = grid.get(source) {
                    let x_plus = Vector2::new(1, 0);
                    let x_minus = Vector2::new(-1, 0);
                    let z_plus = Vector2::new(0, 1);
                    let z_minus = Vector2::new(0, -1);
                    match destination - source {
                        x_plus => node.x_plus.clone(),
                        x_minus => node.x_minus.clone(),
                        z_plus => node.z_plus.clone(),
                        z_minus => node.z_minus.clone(),
                        _ => GraphWeight::Infinity,
                    }
                } else {
                    GraphWeight::Infinity
                }
            }
        }
    }
}
pub struct GraphLayerList<'a> {
    layers: Vec<&'a GraphLayer>,
}
impl<'a> GraphLayerList<'a> {
    pub fn new(layers: Vec<&'a GraphLayer>) -> Self {
        Self { layers }
    }
}
impl<'a> Graph for GraphLayerList<'a> {
    type Node = Vector2<i64>;
    fn get_children(&self, node: &Self::Node) -> Vec<(Self::Node, GraphWeight)> {
        let mut out = vec![];
        for layer in self.layers.iter() {
            out.append(&mut layer.get_children(node));
        }
        return out;
    }
}
pub trait Graph {
    type Node: PartialEq + Eq + std::hash::Hash + Clone;
    /// Gets children of a given node
    fn get_children(&self, node: &Self::Node) -> Vec<(Self::Node, GraphWeight)>;
}
pub struct Path<G: Graph> {
    pub path: Vec<G::Node>,
}

struct Tree {
    children: Vec<(Tree, f32)>,
}

/// Implements Dijkstra's algorythm on a generic graph.
/// used [wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) as a refrence
pub fn dijkstra<'a, G: Graph>(source: G::Node, destination: G::Node, graph: G) -> Path<G> {
    //queue used to priortize searching
    let mut queue = PriorityQueue::new();
    //annotates previous node in shortest path tree. If item is not preseant then previous is marked as infinite.
    let mut previous = HashMap::new();
    //annotates the distance of the node from the source to a given node. If Node is not present then distance can be considered as infinite
    let mut distance = HashMap::<G::Node, GraphWeight>::new();
    //inserting first node
    queue.push(source.clone(), Reverse(GraphWeight::Some(0)));
    distance.insert(source, GraphWeight::Some(0));
    while queue.is_empty() == false {
        let (best_vertex, parent_distance) = queue.pop().unwrap();
        //getting neighbors
        for (child, child_distance) in graph.get_children(&best_vertex).iter() {
            let total_distance = child_distance.clone() + parent_distance.0.clone();
            let is_shortest_path = {
                if let Some(best_known_distance) = distance.get(child) {
                    &total_distance < best_known_distance
                } else {
                    false
                }
            };
            if is_shortest_path {
                distance.insert(child.clone(), total_distance.clone());
                previous.insert(child.clone(), best_vertex.clone());

                queue.push(child.clone(), Reverse(total_distance.into()));
            }
        }
    }

    let mut path: Vec<G::Node> = vec![];
    let mut current = &destination;
    path.push(current.clone());
    loop {
        if let Some(p) = previous.get(current) {
            path.push(p.clone());
            current = p;
        } else {
            return Path {
                path: path.iter().rev().map(|p| p.clone()).collect(),
            };
        }
    }
}
