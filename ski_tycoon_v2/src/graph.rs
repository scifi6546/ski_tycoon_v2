use super::prelude::{Grid, Terrain};
use log::info;
use nalgebra::{Vector2, Vector3};
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum GraphWeight {
    Some(i32),
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
impl std::fmt::Display for GraphWeight {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Infinity => write!(f, "Infinity"),
            Self::Some(v) => write!(f, "Some({})", v),
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
impl std::iter::Sum for GraphWeight {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(GraphWeight::Some(0), |acc, x| acc + x)
    }
}
#[derive(Clone, Debug)]
pub struct GridNode {
    pub x_plus: GraphWeight,
    pub x_minus: GraphWeight,
    pub z_plus: GraphWeight,
    pub z_minus: GraphWeight,
}
#[derive(Clone, Debug)]
pub struct LiftLayer {
    pub start: Node,
    pub end: Node,
    pub weight: GraphWeight,
}
impl LiftLayer {
    pub fn get_children(&self, source: &Node) -> Vec<(Node, GraphWeight)> {
        if source == &self.start {
            info!("getting children of lift");
            vec![(self.end.clone(), self.weight.clone())]
        } else {
            vec![]
        }
    }
    pub fn get(&self, source: Node, destination: Node) -> GraphWeight {
        if source == self.start && destination == self.end {
            self.weight.clone()
        } else {
            GraphWeight::Infinity
        }
    }
}
//layer of graph system
#[derive(Clone, Debug)]
pub enum GraphLayer {
    Grid { grid: Grid<GridNode> },
    Lift(LiftLayer),
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
impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.node.x, self.node.y)
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
                    .cloned()
                    .collect()
                } else {
                    vec![]
                }
            }
            Self::Lift(l) => l.get_children(source),
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
            Self::Lift(l) => l.get(source, destination),
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
    pub fn find_lifts(&self) -> Vec<&'a LiftLayer> {
        self.layers
            .iter()
            .map(|layer| match layer {
                GraphLayer::Grid { .. } => None,
                GraphLayer::Lift(l) => Some(l),
            })
            .filter(|l| l.is_some())
            .map(|l| l.unwrap())
            .collect()
    }
}
impl<'a> Graph for GraphLayerList<'a> {
    fn get_children(&self, node: &Node) -> Vec<(Node, GraphWeight)> {
        let mut out = vec![];
        for layer in self.layers.iter() {
            out.append(&mut layer.get_children(node));
        }
        out
    }
}
impl<'a> Graph for &GraphLayerList<'a> {
    fn get_children(&self, node: &Node) -> Vec<(Node, GraphWeight)> {
        let mut out = vec![];
        for layer in self.layers.iter() {
            out.append(&mut layer.get_children(node));
        }
        out
    }
}
pub trait Graph {
    /// Gets children of a given node
    fn get_children(&self, node: &Node) -> Vec<(Node, GraphWeight)>;
}
#[derive(Clone, Debug, PartialEq)]
pub struct Path {
    pub path: Vec<(Node, GraphWeight)>,
}
impl Path {
    pub fn new(path: Vec<(Node, GraphWeight)>) -> Self {
        Self { path }
    }
    pub fn append(self, other: &Self) -> Self {
        let mut path = vec![];
        for p in self.path.iter() {
            path.push(p.clone());
        }
        for p in other.path.iter() {
            path.push(p.clone());
        }
        Self { path }
    }
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
    pub fn len(&self) -> usize {
        self.path.len()
    }
    pub fn endpoint(&self) -> Option<&Node> {
        if !self.path.is_empty() {
            Some(&self.path[self.path.len() - 1].0)
        } else {
            None
        }
    }
}
impl Default for Path {
    fn default() -> Self {
        Path { path: vec![] }
    }
}
/// Implements Dijkstra's algorythm on a generic graph.
/// used [wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) as a refrence
/// # Preconditions:
/// Graph Weights are greater than zero. If any of the graph weights are less then zero then
/// the alorythm panics
pub fn dijkstra<G: Graph>(source: &Node, destination: &Node, graph: &G) -> Path {
    //queue used to priortize searching
    let mut queue = PriorityQueue::new();
    //annotates previous node in shortest path tree. If item is not preseant then previous is marked as infinite.
    let mut previous = HashMap::new();
    //annotates the distance of the node from the source to a given node. If Node is not present then distance can be considered as infinite
    let mut distance = HashMap::<Node, GraphWeight>::new();
    //inserting first node
    queue.push(source.clone(), Reverse(GraphWeight::Some(0)));
    distance.insert(source.clone(), GraphWeight::Some(0));
    while !queue.is_empty() {
        let (best_vertex, parent_distance) = queue.pop().unwrap();
        //getting neighbors
        for (child, child_distance) in graph.get_children(&best_vertex).iter() {
            assert!(child_distance >= &GraphWeight::Some(0));
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
                previous.insert(child.clone(), (best_vertex.clone(), child_distance.clone()));

                queue.push(child.clone(), Reverse(total_distance));
            }
        }
    }
    let mut path: Vec<(Node, GraphWeight)> = vec![];
    let mut current = (destination.clone(), GraphWeight::Some(0));
    path.push(current.clone());
    loop {
        if let Some((node, weight)) = previous.get(&current.0) {
            path.push((node.clone(), weight.clone().clone()));
            current = (node.clone(), weight.clone().clone());
        } else {
            return Path {
                path: path.iter().rev().cloned().collect(),
            };
        }
    }
}
/// Uses a* to get the shortest distance between a source and destination node
/// heuristic is a function that thaes in (source,destination,graph) and returns an estimated
/// weight
pub fn a_star<G: Graph>(
    source: &Node,
    destination: &Node,
    graph: &G,
    heuristic: Box<dyn Fn(&Node, &Node, &G) -> GraphWeight>,
) -> Path {
    let null_node = Node {
        node: Vector2::new(0, 0),
    };
    #[derive(Clone, Hash, PartialEq, Eq)]
    struct ListEntry {
        node: Node,
        f: GraphWeight,
        g: GraphWeight,
        h: GraphWeight,
        parent: Node,
    }
    let mut open: PriorityQueue<ListEntry, Reverse<GraphWeight>> = PriorityQueue::new();
    let mut closed: HashMap<Node, ListEntry> = HashMap::new();
    let mut visited: HashSet<Node> = HashSet::new();
    {
        let h = heuristic(source, destination, graph);
        open.push(
            ListEntry {
                node: source.clone(),
                g: GraphWeight::Some(0),
                f: h.clone(),
                h: h.clone(),
                parent: null_node,
            },
            Reverse(h),
        );
    }
    while !open.is_empty() {
        let (parent, _) = open.pop().unwrap();
        closed.insert(parent.node.clone(), parent.clone());
        for (child, child_distance) in graph.get_children(&parent.node).iter() {
            let child = child.clone();
            let child_distance = child_distance.clone();

            if !visited.contains(&child) {
                visited.insert(child.clone());
                if &child == destination {
                    let g = child_distance.clone() + parent.g.clone();
                    let h = heuristic(&child, destination, graph);
                    let f = g.clone() + h.clone();
                    closed.insert(
                        child.clone(),
                        ListEntry {
                            node: child.clone(),
                            f,
                            g,
                            h,
                            parent: parent.node.clone(),
                        },
                    );
                    let mut path: Vec<(Node, GraphWeight)> = vec![(child, child_distance)];
                    loop {
                        let (current, _) = path.last().unwrap();

                        let next = closed[current].clone();
                        if &next.parent == source {
                            path.push((next.parent, GraphWeight::Some(0)));
                            return Path {
                                path: (path.iter()).rev().cloned().collect(),
                            };
                        } else {
                            path.push((next.parent, next.g));
                        }
                    }
                } else {
                    let g = child_distance.clone() + parent.g.clone();
                    let h = heuristic(&child, destination, graph);
                    let f = g.clone() + h.clone();
                    open.push(
                        ListEntry {
                            node: child.clone(),
                            f: f.clone(),
                            g,
                            h,
                            parent: parent.node.clone(),
                        },
                        Reverse(f),
                    );
                }
            }
        }
    }

    panic!("node not reachable");
}
/// Path used to follow
#[derive(Clone, Debug, PartialEq)]
pub struct FollowPath {
    pub nodes: Vec<Vector3<f32>>,
    start: Option<Node>,
    endpoint: Option<Node>,
    t: f64,
}
impl FollowPath {
    pub fn new(path: Path, terrain: &Terrain) -> Self {
        let start = if let Some((node, _)) = path.path.get(0) {
            Some(node.clone())
        } else {
            None
        };
        let endpoint = if let Some((node, _)) = path.path.last() {
            Some(node.clone())
        } else {
            None
        };
        Self {
            t: 0.0,
            start,
            endpoint,
            nodes: path
                .path
                .iter()
                .map(|n| terrain.get_transform(&n.0.node).unwrap())
                .collect(),
        }
    }
    pub fn incr(&mut self, incr: f64) {
        self.t += incr
    }
    pub fn start(&self) -> Option<&Node> {
        self.start.as_ref()
    }
    pub fn append(&self, other: &Self) -> Self {
        let t = self.t;
        let mut nodes = self.nodes.clone();
        for i in other.nodes.iter() {
            nodes.push(*i);
        }

        Self {
            t,
            nodes,
            start: self.start.clone(),
            endpoint: if other.endpoint.is_some() {
                other.endpoint.clone()
            } else if self.endpoint.is_some() {
                self.endpoint.clone()
            } else {
                None
            },
        }
    }
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn endpoint(&self) -> Option<&Node> {
        self.endpoint.as_ref()
    }
    pub fn get(&self) -> Vector3<f32> {
        let t0: usize = self.t.floor() as usize;
        if t0 >= self.nodes.len() {
            self.nodes[self.nodes.len() - 1]
        } else if t0 + 1 < self.nodes.len() {
            let x0 = self.nodes[t0];
            let x1 = self.nodes[t0 + 1];
            let t1 = t0 as f64 + 1.0;
            ((x1 - x0) / (t1 as f32 - t0 as f32)) * (self.t as f32 - t0 as f32) + (x0)
        } else {
            self.nodes[t0]
        }
    }
    pub fn at_end(&self) -> bool {
        self.t > self.nodes.len() as f64
    }
}
pub mod graph_debug {
    use super::GraphLayer;
    use egui::CtxRef;
    use legion::*;
    pub fn terrain_debug_window(world: &World, context: &mut CtxRef) {
        egui::Window::new("terrain").show(context, |ui| {
            let mut query = <&GraphLayer>::query();
            for layer in query.iter(world) {
                match layer {
                    GraphLayer::Grid { grid } => {
                        ui.label(format!(
                            "Grid, width: {} height: {}",
                            grid.width(),
                            grid.height()
                        ));
                    }
                    GraphLayer::Lift(lift) => {
                        ui.label(format!("Lift start: {} end: {}", lift.start, lift.end));
                    }
                }
            }
        });
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
