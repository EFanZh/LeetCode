use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(node: &RefCell<TreeNode>) -> (u32, u32) {
        let node = &*node.borrow();
        let (left_count, left_max) = node.left.as_deref().map_or((0, 0), Self::dfs);
        let (right_count, right_max) = node.right.as_deref().map_or((0, 0), Self::dfs);
        let child_count = left_count + right_count;
        let child_max = left_max.max(right_max);
        let val = node.val.cast_unsigned();

        if val < child_max {
            (child_count, child_max)
        } else {
            (child_count + 1, val)
        }
    }

    pub fn count_dominant_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.as_deref().map_or((0, 0), Self::dfs).0.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_dominant_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_dominant_nodes(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
