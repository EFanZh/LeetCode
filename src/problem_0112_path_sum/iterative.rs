use super::super::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> bool {
        if let Some(mut node) = root {
            let mut stack = Vec::new();

            loop {
                let (val, left, right) = {
                    let node_ref = node.borrow();

                    (node_ref.val, node_ref.left.clone(), node_ref.right.clone())
                };

                match (left, right) {
                    (None, None) => {
                        if val == sum {
                            return true;
                        } else if let Some((next_node, next_sum)) = stack.pop() {
                            node = next_node;
                            sum = next_sum;
                        } else {
                            break;
                        }
                    }
                    (None, Some(child)) | (Some(child), None) => {
                        node = child;
                        sum -= val;
                    }
                    (Some(left), Some(right)) => {
                        node = left;
                        sum -= val;

                        stack.push((right, sum));
                    }
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::has_path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
