use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn generate_trees_helper(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if start == end {
            vec![None]
        } else {
            let mut result = Vec::new();

            for val in start..end {
                let left_result = Self::generate_trees_helper(start, val);
                let right_result = Self::generate_trees_helper(val + 1, end);

                for left in left_result {
                    for right in &right_result {
                        result.push(Some(Rc::new(RefCell::new(TreeNode {
                            val,
                            left: left.clone(),
                            right: right.clone(),
                        }))));
                    }
                }
            }

            result
        }
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            Vec::new()
        } else {
            Self::generate_trees_helper(1, n + 1)
        }
    }
}

impl super::Solution for Solution {
    fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        Self::generate_trees(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
