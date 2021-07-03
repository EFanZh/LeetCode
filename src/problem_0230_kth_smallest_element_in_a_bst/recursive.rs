use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn kth_smallest_helper(root: Option<&RefCell<TreeNode>>, k: i32) -> Result<i32, i32> {
        root.map_or_else(
            || Err(k),
            |node| {
                let node = node.borrow();

                Self::kth_smallest_helper(node.left.as_deref(), k).or_else(|x| {
                    if x == 0 {
                        Ok(node.val)
                    } else {
                        Self::kth_smallest_helper(node.right.as_deref(), x - 1)
                    }
                })
            },
        )
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_smallest_helper(root.as_deref(), k - 1).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_smallest(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
