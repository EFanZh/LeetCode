use super::super::data_structures::TreeNode;

pub struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn generate_trees_helper(start: i32, end: i32, f: &mut dyn FnMut(Option<Rc<RefCell<TreeNode>>>)) {
        if start == end {
            f(None);
        } else {
            for val in start..end {
                Self::generate_trees_helper(start, val, &mut |left| {
                    Self::generate_trees_helper(val + 1, end, &mut |right| {
                        f(Some(Rc::new(RefCell::new(TreeNode {
                            val,
                            left: left.clone(),
                            right,
                        }))))
                    })
                });
            }
        }
    }

    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = Vec::new();

        if n > 0 {
            Self::generate_trees_helper(1, n + 1, &mut |item| result.push(item));
        }

        result
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
