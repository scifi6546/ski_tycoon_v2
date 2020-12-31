use super::prelude::Grid;
use log::info;
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
                Self::Some(_) => std::cmp::Ordering::Greater,
            },
            Self::Some(s) => match other {
                Self::Infinity => std::cmp::Ordering::Less,
                Self::Some(o) => s.cmp(o),
            },
        }
    }
}
#[derive(Clone, Debug)]
pub struct GridNode {
    pub x_plus: GraphWeight,
    pub x_minus: GraphWeight,
    pub z_plus: GraphWeight,
    pub z_minus: GraphWeight,
}
//layer of graph system
#[derive(Clone, Debug)]
pub enum GraphLayer {
    Grid { grid: Grid<GridNode> },
}
impl GraphLayer {
    pub fn get_children(&self, source: &Vector2<i64>) -> Vec<(Vector2<i64>, GraphWeight)> {
        info!("get children grid??");
        match self {
            Self::Grid { grid } => {
                if let Some(node) = grid.get(source.clone()) {
                    info!("getting node: {:?} from grid at {}", node, source);
                    vec![
                        (source + Vector2::new(1, 0), node.x_plus.clone()),
                        (source + Vector2::new(-1, 0), node.x_minus.clone()),
                        (source + Vector2::new(0, 1), node.z_plus.clone()),
                        (source + Vector2::new(0, -1), node.z_minus.clone()),
                    ]
                } else {
                    info!("source: {} does not exist", source);
                    vec![]
                }
            }
        }
    }
    pub fn get(&self, source: Vector2<i64>, destination: Vector2<i64>) -> GraphWeight {
        info!("source: {} destination: {}", source, destination);
        match self {
            Self::Grid { grid } => {
                if let Some(node) = grid.get(source) {
                    let x_plus = Vector2::new(1, 0);
                    let x_minus = Vector2::new(-1, 0);
                    let z_plus = Vector2::new(0, 1);
                    let z_minus = Vector2::new(0, -1);
                    let delta = destination - source;
                    info!("delta: {}", delta);
                    if delta == x_plus {
                        node.x_plus.clone()
                    } else if delta == x_minus {
                        node.x_minus.clone()
                    } else if delta == z_plus {
                        node.z_plus.clone()
                    } else if delta == z_minus {
                        node.z_minus.clone()
                    } else {
                        GraphWeight::Infinity
                    }
                } else {
                    GraphWeight::Infinity
                }
            }
        }
    }
}
pub struct GraphLayerList {
    layers: Vec<GraphLayer>,
}
impl GraphLayerList {
    pub fn new(layers: Vec<GraphLayer>) -> Self {
        Self { layers }
    }
}
impl Graph for GraphLayerList {
    type Node = Vector2<i64>;
    type NodeFloat = Vector2<f64>;
    fn to_node_float(node: Self::Node) -> Self::NodeFloat {
        Vector2::new(node.x as f64, node.y as f64)
    }
    fn get_children(&self, node: &Self::Node) -> Vec<(Self::Node, GraphWeight)> {
        info!("getting children of {}", node);
        let mut out = vec![];
        for layer in self.layers.iter() {
            out.append(&mut layer.get_children(node));
        }
        for (node, weight) in out.iter() {
            info!("child node: {} weight: {:?}", node, weight);
        }
        return out;
    }
}
pub trait Graph {
    type Node: PartialEq
        + Eq
        + std::hash::Hash
        + Clone
        + std::fmt::Debug
        + std::fmt::Display
        + std::ops::Sub<Output = Self::Node>;
    type NodeFloat: PartialEq
        + Clone
        + std::fmt::Debug
        + std::fmt::Display
        + std::ops::Add<Output = Self::NodeFloat>
        + std::ops::Sub<Output = Self::NodeFloat>
        + std::ops::Div<f64, Output = Self::NodeFloat>
        + std::ops::Mul<f64, Output = Self::NodeFloat>;
    fn to_node_float(node: Self::Node) -> Self::NodeFloat;
    /// Gets children of a given node
    fn get_children(&self, node: &Self::Node) -> Vec<(Self::Node, GraphWeight)>;
}
#[derive(Clone, Debug)]
pub struct Path<G: Graph> {
    pub path: Vec<G::Node>,
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
                    true
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
    info!("previous path: ");
    for (k, v) in previous.iter() {
        info!("k: {:?} v: {:?}", k, v);
    }
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
/// Path used to follow
#[derive(Clone, Debug)]
pub struct FollowPath<G: Graph> {
    path: Path<G>,
    t: f64,
}
impl<G: Graph> FollowPath<G> {
    pub fn new(path: Path<G>) -> Self {
        Self { path, t: 0.0 }
    }
    pub fn incr(&mut self, incr: f64) {
        self.t += incr
    }
    pub fn get(&self) -> G::NodeFloat {
        let t0: usize = self.t.floor() as usize;
        if t0 >= self.path.path.len() {
            G::to_node_float(self.path.path[self.path.path.len() - 1].clone())
        } else {
            if t0 + 1 < self.path.path.len() {
                let x0 = self.path.path[t0].clone();
                let x1 = self.path.path[t0 + 1].clone();
                let t1 = t0 as f64 + 1.0;
                (G::to_node_float(x1 - x0.clone()) / (t1 - t0 as f64)) * (self.t - t0 as f64)
                    + G::to_node_float(x0.clone())
            } else {
                G::to_node_float(self.path.path[t0].clone())
            }
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn graph_ord() {
        let inf = GraphWeight::Infinity;
        let zero = GraphWeight::Some(0);
        assert!(inf > zero);
        assert!(inf >= inf);
        assert!(inf == inf);
        assert!(zero < inf);
        assert!(zero <= inf);
    }
}
