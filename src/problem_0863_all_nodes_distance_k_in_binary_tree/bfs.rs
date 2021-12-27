use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

enum Direction {
    Void,
    Left,
    Right,
    Parent,
}

impl Solution {
    fn get_parents(
        node: &Rc<RefCell<TreeNode>>,
        parents: &mut HashMap<*const RefCell<TreeNode>, (Rc<RefCell<TreeNode>>, Direction)>,
    ) {
        let node_ref = node.borrow();

        if let Some(left) = &node_ref.left {
            parents.insert(Rc::as_ptr(left), (Rc::clone(node), Direction::Left));

            Self::get_parents(left, parents);
        }

        if let Some(right) = &node_ref.right {
            parents.insert(Rc::as_ptr(right), (Rc::clone(node), Direction::Right));

            Self::get_parents(right, parents);
        }
    }

    pub fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        let root = root.unwrap();
        let target = target.unwrap();
        let mut parents = HashMap::new();

        Self::get_parents(&root, &mut parents);

        let mut queue = VecDeque::from(vec![(target, Direction::Void)]);

        for _ in 0..k {
            for _ in 0..queue.len() {
                let (node, direction) = queue.pop_front().unwrap();
                let node_ref = node.borrow();

                match direction {
                    Direction::Void => {
                        if let Some(parent) = parents.remove(&Rc::as_ptr(&node)) {
                            queue.push_back(parent);
                        }

                        if let Some(left) = node_ref.left.clone() {
                            queue.push_back((left, Direction::Parent));
                        }

                        if let Some(right) = node_ref.right.clone() {
                            queue.push_back((right, Direction::Parent));
                        }
                    }
                    Direction::Left => {
                        if let Some(parent) = parents.remove(&Rc::as_ptr(&node)) {
                            queue.push_back(parent);
                        }

                        if let Some(right) = node_ref.right.clone() {
                            queue.push_back((right, Direction::Parent));
                        }
                    }
                    Direction::Right => {
                        if let Some(parent) = parents.remove(&Rc::as_ptr(&node)) {
                            queue.push_back(parent);
                        }

                        if let Some(left) = node_ref.left.clone() {
                            queue.push_back((left, Direction::Parent));
                        }
                    }
                    Direction::Parent => {
                        if let Some(left) = node_ref.left.clone() {
                            queue.push_back((left, Direction::Parent));
                        }

                        if let Some(right) = node_ref.right.clone() {
                            queue.push_back((right, Direction::Parent));
                        }
                    }
                }
            }
        }

        queue.into_iter().map(|(node, _)| node.borrow().val).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn distance_k(root: Option<Rc<RefCell<TreeNode>>>, target: Option<Rc<RefCell<TreeNode>>>, k: i32) -> Vec<i32> {
        Self::distance_k(root, target, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
