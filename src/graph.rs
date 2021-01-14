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
impl GraphWeight {
    pub fn is_finite(&self) -> bool {
        match self {
            GraphWeight::Some(_) => true,
            GraphWeight::Infinity => false,
        }
    }
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
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Node {
    pub node: Vector2<i64>,
}
impl Node {
    pub fn to_node_float(&self) -> NodeFloat {
        NodeFloat {
            node: Vector2::new(self.node.x as f32, self.node.y as f32),
        }
    }
}
impl std::ops::Sub for Node {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            node: self.node - other.node,
        }
    }
}
pub struct NodeFloat {
    pub node: Vector2<f32>,
}
impl std::ops::Div<f64> for NodeFloat {
    type Output = Self;
    fn div(self, other: f64) -> Self {
        Self {
            node: self.node / other as f32,
        }
    }
}
impl std::ops::Mul<f64> for NodeFloat {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            node: self.node * other as f32,
        }
    }
}
impl std::ops::Add for NodeFloat {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            node: self.node + other.node,
        }
    }
}
impl GraphLayer {
    pub fn get_children(&self, source: &Node) -> Vec<(Node, GraphWeight)> {
        match self {
            Self::Grid { grid } => {
                if let Some(node) = grid.get(source.node) {
                    vec![
                        (
                            Node {
                                node: source.node + Vector2::new(1, 0),
                            },
                            node.x_plus.clone(),
                        ),
                        (
                            Node {
                                node: source.node + Vector2::new(-1, 0),
                            },
                            node.x_minus.clone(),
                        ),
                        (
                            Node {
                                node: source.node + Vector2::new(0, 1),
                            },
                            node.z_plus.clone(),
                        ),
                        (
                            Node {
                                node: source.node + Vector2::new(0, -1),
                            },
                            node.z_minus.clone(),
                        ),
                    ]
                    .iter()
                    .filter(|(_, weight)| weight.is_finite())
                    .map(|v| v.clone())
                    .collect()
                } else {
                    vec![]
                }
            }
        }
    }
    pub fn get(&self, source: Node, destination: Node) -> GraphWeight {
        info!("source: {:?} destination: {:?}", source, destination);
        match self {
            Self::Grid { grid } => {
                if let Some(node) = grid.get(source.node) {
                    let x_plus = Vector2::new(1, 0);
                    let x_minus = Vector2::new(-1, 0);
                    let z_plus = Vector2::new(0, 1);
                    let z_minus = Vector2::new(0, -1);
                    let delta = destination - source;
                    if delta.node == x_plus {
                        node.x_plus.clone()
                    } else if delta.node == x_minus {
                        node.x_minus.clone()
                    } else if delta.node == z_plus {
                        node.z_plus.clone()
                    } else if delta.node == z_minus {
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
pub struct GraphLayerList<'a> {
    layers: Vec<&'a GraphLayer>,
}
impl<'a> GraphLayerList<'a> {
    pub fn new(layers: Vec<&'a GraphLayer>) -> Self {
        Self { layers }
    }
}
impl<'a> Graph for GraphLayerList<'a> {
    fn get_children(&self, node: &Node) -> Vec<(Node, GraphWeight)> {
        let mut out = vec![];
        for layer in self.layers.iter() {
            out.append(&mut layer.get_children(node));
        }
        return out;
    }
}
pub trait Graph {
    /// Gets children of a given node
    fn get_children(&self, node: &Node) -> Vec<(Node, GraphWeight)>;
}
#[derive(Clone, Debug)]
pub struct Path {
    pub path: Vec<Node>,
}
/// Implements Dijkstra's algorythm on a generic graph.
/// used [wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) as a refrence
pub fn dijkstra<'a, G: Graph>(source: Node, destination: Node, graph: G) -> Path {
    //queue used to priortize searching
    let mut queue = PriorityQueue::new();
    //annotates previous node in shortest path tree. If item is not preseant then previous is marked as infinite.
    let mut previous = HashMap::new();
    //annotates the distance of the node from the source to a given node. If Node is not present then distance can be considered as infinite
    let mut distance = HashMap::<Node, GraphWeight>::new();
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
    let mut path: Vec<Node> = vec![];
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
/// Finds the best path while searching `search_size` nodes
/// current issues: Having skiiers avoid punishments just results in them taking a really short hop rather then trying to go far out
///
pub fn find_best_path<'a, G: Graph>(
    source: Node,
    destination: Node,
    search_size: usize,
    graph: G,
) -> Path {
    let mut nodes_processed = 0;
    //queue used to priortize searching
    let mut queue = PriorityQueue::new();
    //annotates previous node in shortest path tree. If item is not preseant then previous is marked as infinite.
    let mut previous = HashMap::new();
    //annotates the distance of the node from the source to a given node. If Node is not present then distance can be considered as infinite
    let mut distance = HashMap::<Node, GraphWeight>::new();
    let mut distance_priority: PriorityQueue<Node, Reverse<GraphWeight>> = PriorityQueue::new();
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
                distance_priority.push(child.clone(), Reverse(total_distance.clone().into()));
                queue.push(child.clone(), Reverse(total_distance.into()));
            }
            nodes_processed += 1;
            if nodes_processed == search_size {
                break;
            }
        }
    }
    let mut path: Vec<Node> = vec![];
    let (mut current, _) = distance_priority.pop().unwrap();
    path.push(current.clone());
    loop {
        if let Some(p) = previous.get(&current) {
            path.push(p.clone());
            current = p.clone();
        } else {
            return Path {
                path: path.iter().rev().map(|p| p.clone()).collect(),
            };
        }
    }
    todo!()
}
/// Path used to follow
#[derive(Clone, Debug)]
pub struct FollowPath {
    path: Path,
    t: f64,
}
impl FollowPath {
    pub fn new(path: Path) -> Self {
        Self { path, t: 0.0 }
    }
    pub fn incr(&mut self, incr: f64) {
        self.t += incr
    }
    pub fn get(&self) -> NodeFloat {
        let t0: usize = self.t.floor() as usize;
        if t0 >= self.path.path.len() {
            self.path.path[self.path.path.len() - 1].to_node_float()
        } else {
            if t0 + 1 < self.path.path.len() {
                let x0 = self.path.path[t0].clone();
                let x1 = self.path.path[t0 + 1].clone();
                let t1 = t0 as f64 + 1.0;
                ((x1 - x0.clone()).to_node_float() / (t1 - t0 as f64)) * (self.t - t0 as f64)
                    + (x0.clone()).to_node_float()
            } else {
                (self.path.path[t0].clone()).to_node_float()
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
