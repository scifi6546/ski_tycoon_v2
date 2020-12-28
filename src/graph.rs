use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
pub trait Graph {
    type Node: PartialEq + Eq + std::hash::Hash + Ord + Clone;
    /// Gets children of a given node
    fn get_children(&self, node: &Self::Node) -> Vec<(Self::Node, u32)>;
}
pub struct Path<G: Graph> {
    path: Vec<G::Node>,
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
    let mut distance = HashMap::<G::Node, u32>::new();
    //inserting first node
    queue.push(source.clone(), Reverse(0u32));
    distance.insert(source, 0);
    while queue.is_empty() == false {
        let (best_vertex, parent_distance) = queue.pop().unwrap();
        //getting neighbors
        for (child, child_distance) in graph.get_children(&best_vertex).iter() {
            let total_distance = child_distance.clone() + parent_distance.0;
            let is_shortest_path = {
                if let Some(best_known_distance) = distance.get(child) {
                    &total_distance < best_known_distance
                } else {
                    false
                }
            };
            if is_shortest_path {
                distance.insert(child.clone(), total_distance);
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
