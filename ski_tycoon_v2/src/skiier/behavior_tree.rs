use super::{
    super::prelude::{a_star, GraphWeight, Path},
    FollowPath, GraphLayerList, Node, Terrain,
};
use log::error;
#[derive(Clone, Debug, PartialEq)]
pub struct Decision {
    pub cost: Number<f32>,
    pub name: String,
    pub path: FollowPath,
    pub endpoint: Node,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Number<T> {
    Infinite,
    Finite(T),
}
impl<T: std::ops::Add<Output = T>> std::ops::Add for Number<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self {
            Self::Infinite => Self::Infinite,
            Self::Finite(n1) => match other {
                Self::Infinite => Self::Infinite,
                Self::Finite(n2) => Self::Finite(n1 + n2),
            },
        }
    }
}
impl<T: std::fmt::Display> std::fmt::Display for Number<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Infinite => write!(f, "Number::Infinite"),
            Self::Finite(n) => write!(f, " Self::Finite({})", n),
        }
    }
}
impl<T: std::cmp::PartialOrd> std::cmp::PartialOrd for Number<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Infinite => match other {
                Self::Infinite => Some(std::cmp::Ordering::Equal),
                Self::Finite(_) => Some(std::cmp::Ordering::Greater),
            },
            Self::Finite(n1) => match other {
                Self::Infinite => Some(std::cmp::Ordering::Less),
                Self::Finite(n2) => n1.partial_cmp(n2),
            },
        }
    }
}
pub trait TreeNode {
    fn cost(&self, layers: &GraphLayerList, position: Node, terrain: &Terrain) -> Decision;
    fn children(&self) -> Vec<Box<dyn TreeNode>>;
    fn name(&self) -> String;
    fn best_path(
        &self,
        search_length: usize,
        layers: &GraphLayerList,
        position: Node,
        terrain: &Terrain,
    ) -> Vec<Decision> {
        if search_length == 0 {
            vec![self.cost(layers, position, terrain)]
        } else {
            let self_cost = self.cost(layers, position, terrain);
            let mut best_weight = Number::Infinite;
            let mut best_path = vec![];
            for child in self.children().iter() {
                let mut child_path = child.best_path(
                    search_length - 1,
                    layers,
                    self_cost.clone().endpoint.clone(),
                    terrain,
                );

                let child_weight = child_path
                    .iter()
                    .fold(Number::Finite(0.0), |acc, d| acc + d.cost.clone());

                if child_weight <= best_weight {
                    best_path = vec![self_cost.clone()];
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
    fn name(&self) -> String {
        "Up".to_string()
    }
    fn cost(&self, layers: &GraphLayerList, position: Node, terrain: &Terrain) -> Decision {
        let lift_list = layers.find_lifts();
        let (cost, best_path) = lift_list
            .iter()
            .map(|lift| {
                const LIFT_COST: f32 = 1.0;
                if position == lift.start {
                    let path = Path::new(vec![
                        (lift.start.clone(), GraphWeight::Some(0)),
                        (lift.end.clone(), GraphWeight::Some(1)),
                    ]);
                    (Number::Finite(LIFT_COST), path)
                } else {
                    (Number::Infinite, Path::default())
                }
            })
            .fold(
                (Number::Infinite, Path::default()),
                |(acc_cost, acc_path), (other_cost, other_path)| {
                    if acc_cost > other_cost {
                        (other_cost, other_path)
                    } else {
                        (acc_cost, acc_path)
                    }
                },
            );
        Decision {
            cost,
            endpoint: if let Some(point) = best_path.endpoint() {
                point.clone()
            } else {
                position
            },
            path: FollowPath::new(best_path, terrain),
            name: self.name(),
        }
    }
    fn children(&self) -> Vec<Box<dyn TreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
}
pub struct Down {}
impl Down {
    fn heuristic(start: &Node, end: &Node, _graph: &GraphLayerList) -> GraphWeight {
        GraphWeight::Some(
            ((end.node.x - start.node.x).abs() + (end.node.y - start.node.y).abs()) as i32,
        )
    }
}
impl TreeNode for Down {
    fn name(&self) -> String {
        "Down".to_string()
    }
    fn cost(&self, layers: &GraphLayerList, position: Node, terrain: &Terrain) -> Decision {
        let lift_list = layers.find_lifts();
        let (cost, best_path) = lift_list
            .iter()
            .map(|lift| {
                let path_to_lift =
                    a_star(&position, &lift.start, layers, Box::new(Self::heuristic));
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
            .fold(
                (Number::Infinite, Path::default()),
                |acc, (x_num, x_data)| {
                    if acc.0 > Number::Finite(x_num as f32) {
                        (Number::Finite(x_num as f32), x_data)
                    } else {
                        acc
                    }
                },
            );
        Decision {
            cost: if best_path.len() > 1 {
                cost
            } else {
                Number::Infinite
            },
            endpoint: if let Some(point) = best_path.endpoint() {
                point.clone()
            } else {
                position
            },
            path: FollowPath::new(best_path, terrain),
            name: self.name(),
        }
    }
    fn children(&self) -> Vec<Box<dyn TreeNode>> {
        vec![Box::new(Up {}), Box::new(Down {})]
    }
}
pub struct SearchStart {}
impl Default for SearchStart {
    fn default() -> Self {
        SearchStart {}
    }
}
impl TreeNode for SearchStart {
    fn name(&self) -> String {
        "Search Start".to_string()
    }
    fn cost(&self, _layers: &GraphLayerList, position: Node, terrain: &Terrain) -> Decision {
        Decision {
            cost: Number::Finite(0.0),
            endpoint: position,
            path: FollowPath::new(Path::default(), terrain),
            name: self.name(),
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
        fn name(&self) -> String {
            "A".to_string()
        }
        fn cost(&self, _layers: &GraphLayerList, position: Node) -> Decision {
            Decision {
                cost: Number::Finite(5.0),
                path: FollowPath::new(Path { path: vec![] }),
                endpoint: position,
                name: self.name(),
            }
        }
        fn children(&self) -> Vec<Box<dyn TreeNode>> {
            vec![Box::new(A {}), Box::new(B {})]
        }
    }
    impl TreeNode for B {
        fn name(&self) -> String {
            "B".to_string()
        }
        fn cost(&self, _layers: &GraphLayerList, position: Node) -> Decision {
            Decision {
                cost: Number::Finite(15.0),
                path: FollowPath::new(Path { path: vec![] }),
                endpoint: position,
                name: self.name(),
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
                    cost: Number::Finite(5.0),
                    path: FollowPath::new(Path { path: vec![] }),
                    endpoint: Node {
                        node: Vector2::new(0, 0),
                    },
                    name: "A".to_string(),
                }
            );
        }
    }
    #[test]
    fn number_comps() {
        let inf = Number::Infinite;
        let fin_0 = Number::Finite(0.0f32);
        let fin_5 = Number::Finite(5.0f32);
        assert_ne!(inf, fin_0);
        assert!(inf > fin_0);
        assert!(inf >= fin_0);
        assert!(fin_0 < inf);
        assert!(fin_0 <= inf);
        assert!(fin_5 < inf);
        assert!(fin_5 <= inf);
        assert_ne!(fin_0, fin_5);
        assert!(fin_0 < fin_5);
        assert!(fin_0 <= fin_5);
    }
}
