use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if let Some(node) = root {
            let mut node_level = VecDeque::new();

            result.push(vec![node.borrow().val]);

            node_level.push_back(node);

            loop {
                let mut value_level = Vec::new();

                for _ in 0..node_level.len() {
                    let node = node_level.pop_front().unwrap();
                    let node_ref = node.borrow();

                    if let Some(left) = &node_ref.left {
                        node_level.push_back(left.clone());
                        value_level.push(left.borrow().val);
                    }

                    if let Some(right) = &node_ref.right {
                        node_level.push_back(right.clone());
                        value_level.push(right.borrow().val);
                    };
                }

                if node_level.is_empty() {
                    break;
                } else {
                    result.push(value_level);
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::level_order(root)
    }
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
