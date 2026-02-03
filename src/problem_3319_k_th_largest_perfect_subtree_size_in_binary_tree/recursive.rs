use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::cmp::Reverse;
use std::rc::Rc;

impl Solution {
    fn check(node: Option<&RefCell<TreeNode>>, depths: &mut Vec<u32>) -> u32 {
        node.map_or(0, |node| {
            let node = node.borrow();
            let left_depth = Self::check(node.left.as_deref(), depths);
            let right_depth = Self::check(node.right.as_deref(), depths);

            if left_depth == right_depth && left_depth != u32::MAX {
                let depth = left_depth + 1;

                depths.push(depth);

                depth
            } else {
                u32::MAX
            }
        })
    }

    pub fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let k = k.cast_unsigned() as usize;
        let mut sizes = Vec::new();

        Self::check(root.as_deref(), &mut sizes);

        let index = k - 1;

        if index < sizes.len() {
            (1 << *sizes.select_nth_unstable_by_key(index, |&depth| Reverse(depth)).1) - 1
        } else {
            -1
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        Self::kth_largest_perfect_subtree(root, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
