use super::{
    super::prelude::{dijkstra, GraphWeight, Path},
    FollowPath, GraphLayerList, Node,
};
use log::error;
#[derive(Clone, Debug, PartialEq)]
pub struct Decision {
    pub cost: f32,
    pub path: FollowPath,
    pub endpoint: Node,
}
pub trait TreeNode {
    fn cost(&self, layers: &GraphLayerList, position: Node) -> Decision;
    fn children(&self) -> Vec<Box<dyn TreeNode>>;
    fn best_path(
        &self,
        search_length: usize,
        layers: &GraphLayerList,
        position: Node,
    ) -> Vec<Decision> {
        if search_length == 0 {
            vec![self.cost(layers, position)]
        } else {
            let self_cost = self.cost(layers, position);
            let mut best_weight = f32::MAX;
            let mut best_path = vec![];
            for child in self.children().iter() {
                let mut child_path = child.best_path(
                    search_length - 1,
                    layers,
                    self_cost.clone().endpoint.clone(),
                );
                let child_weight = child_path.iter().fold(0.0, |acc, d| acc + d.cost);
                if child_weight < best_weight {
                    best_path = vec![];
                    best_path.push(self_cost.clone());
                    best_path.append(&mut child_path);
                    best_weight = child_weight;
                }
            }
            best_path
        }
    }
}
pub struct Up {}
impl TreeNode for Up {
    fn cost(&self, layers: &GraphLayerList, position: Node) -> Decision {
        let lift_list = layers.find_lifts();
        let (cost, best_path) = lift_list
            .iter()
            .map(|lift| {
                const LIFT_COST: i32 = 1;
                let path_to_lift = dijkstra(&position, &lift.start, layers).append(&Path {
                    path: vec![(lift.end.clone(), GraphWeight::Some(1))],
                });
                let path_cost: GraphWeight = path_to_lift
                    .path
                    .iter()
                    .map(|(_, weight)| weight.clone())
                    .sum();
                let total_cost = match path_cost {
                    GraphWeight::Infinity => {
                        error!("invalid graph weight");
                        panic!()
                    }
                    GraphWeight::Some(n) => n + LIFT_COST,
                };
                (total_cost, path_to_lift)
            })
            .fold((i32::MAX, Path::default()), |acc, x| {
                if acc.0 > x.0 {
                    x
                } else {
                    acc
                }
            });
        Decision {
            cost: cost as f32,
            endpoint: best_path.endpoint().clone(),
            path: FollowPath::new(best_path),
        }
    }
    fn children(&self) -> Vec<Box<dyn TreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
}
pub struct Down {}
impl TreeNode for Down {
    fn cost(&self, layers: &GraphLayerList, position: Node) -> Decision {
        let lift_list = layers.find_lifts();
        let (cost, best_path) = lift_list
            .iter()
            .map(|lift| {
                let path_to_lift = dijkstra(&position, &lift.start, layers);
                let path_cost: GraphWeight = path_to_lift
                    .path
                    .iter()
                    .map(|(_, weight)| weight.clone())
                    .sum();
                let total_cost = match path_cost {
                    GraphWeight::Infinity => {
                        error!("invalid graph weight");
                        panic!()
                    }
                    GraphWeight::Some(n) => n,
                };
                (total_cost, path_to_lift)
            })
            .fold((i32::MAX, Path::default()), |acc, x| {
                if acc.0 > x.0 {
                    x
                } else {
                    acc
                }
            });
        Decision {
            cost: cost as f32,
            endpoint: best_path.endpoint().clone(),
            path: FollowPath::new(best_path),
        }
    }
    fn children(&self) -> Vec<Box<dyn TreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
}
pub struct SearchStart {}
impl Default for SearchStart{
    fn default()->Self{
        SearchStart{}
    }
}
impl TreeNode for SearchStart {
    fn cost(&self, _layers: &GraphLayerList, position: Node) -> Decision {
        Decision {
            cost:0.0,
            endpoint: position,
            path: FollowPath::new(Path::default())
        }
    }
    fn children(&self) -> Vec<Box<dyn TreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
}
#[cfg(test)]
mod test {
    use super::super::super::prelude::Path;
    use super::*;
    use nalgebra::Vector2;
    struct A {}
    impl TreeNode for A {
        fn cost(&self, _layers: &GraphLayerList, position: Node) -> Decision {
            Decision {
                cost: 5.0,
                path: FollowPath::new(Path { path: vec![] }),
                endpoint: position,
            }
        }
        fn children(&self) -> Vec<Box<dyn TreeNode>> {
            vec![Box::new(A {}), Box::new(B {})]
        }
    }
    impl TreeNode for B {
        fn cost(&self, _layers: &GraphLayerList, position: Node) -> Decision {
            Decision {
                cost: 15.0,
                path: FollowPath::new(Path { path: vec![] }),
                endpoint: position,
            }
        }
        fn children(&self) -> Vec<Box<dyn TreeNode>> {
            vec![Box::new(A {}), Box::new(B {})]
        }
    }
    struct B {}
    #[test]
    fn simple_create() {
        let a = A {};
        let p = a.best_path(
            4,
            &GraphLayerList::new(vec![]),
            Node {
                node: Vector2::new(0, 0),
            },
        );
        for i in 0..4 {
            assert_eq!(
                p[i],
                Decision {
                    cost: 5.0,
                    path: FollowPath::new(Path { path: vec![] }),
                    endpoint: Node {
                        node: Vector2::new(0, 0),
                    },
                }
            );
        }
    }
}
