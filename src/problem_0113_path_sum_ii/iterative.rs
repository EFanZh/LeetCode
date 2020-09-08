use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

enum Frame {
    Left(Rc<RefCell<TreeNode>>, i32),
    Right,
}

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, mut sum: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        if let Some(mut node) = root {
            let mut base = Vec::new();
            let mut stack = Vec::new();

            loop {
                let (val, left, right) = {
                    let node_ref = node.borrow();

                    (node_ref.val, node_ref.left.clone(), node_ref.right.clone())
                };

                match (left, right) {
                    (None, None) => {
                        if val == sum {
                            base.push(val);

                            result.push(base.clone());

                            base.pop();
                        }
                    }
                    (None, Some(child)) | (Some(child), None) => {
                        base.push(val);

                        node = child;
                        sum -= val;

                        stack.push(Frame::Right);

                        continue;
                    }
                    (Some(left), Some(right)) => {
                        base.push(val);

                        node = left;
                        sum -= val;

                        stack.push(Frame::Left(right, sum));

                        continue;
                    }
                }

                loop {
                    match stack.pop() {
                        None => return result,
                        Some(Frame::Left(right, saved_sum)) => {
                            node = right;
                            sum = saved_sum;

                            stack.push(Frame::Right);

                            break;
                        }
                        Some(Frame::Right) => {
                            base.pop();
                        }
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> Vec<Vec<i32>> {
        Self::path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
