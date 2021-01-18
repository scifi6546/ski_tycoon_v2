use super::{FollowPath, GraphLayerList, Node};
#[derive(Clone, Debug, PartialEq)]
struct Decision {
    cost: f32,
    path: FollowPath,
    endpoint: Node,
}
trait TreeNode {
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
#[cfg(test)]
mod test {
    use super::super::Path;
    use super::*;
    use nalgebra::Vector2;
    struct A {}
    impl TreeNode for A {
        fn cost(&self, layers: &GraphLayerList, position: Node) -> Decision {
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
        fn cost(&self, layers: &GraphLayerList, position: Node) -> Decision {
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
