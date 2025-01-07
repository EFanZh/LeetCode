use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn get_depth(node: Option<&RefCell<TreeNode>>) -> u32 {
        node.map_or(0, |node| {
            let node = node.borrow();
            let node = &*node;

            u32::max(
                Self::get_depth(node.left.as_deref()),
                Self::get_depth(node.right.as_deref()),
            ) + 1
        })
    }

    fn helper(node: Option<&RefCell<TreeNode>>, start: i32, result: &mut u32) -> (bool, u32) {
        node.map_or((false, 0), |node| {
            let node = node.borrow();
            let node = &*node;
            let (has_start, left_depth) = Self::helper(node.left.as_deref(), start, result);

            let (candidate, depth) = 'block: {
                let (start_branch_depth, other_branch_depth) = if has_start {
                    let right_depth = Self::get_depth(node.right.as_deref());

                    (left_depth, right_depth)
                } else {
                    let (has_start, right_depth) = Self::helper(node.right.as_deref(), start, result);

                    if has_start {
                        (right_depth, left_depth)
                    } else {
                        let depth = u32::max(left_depth, right_depth);

                        if node.val == start {
                            break 'block (depth, 1);
                        }

                        return (false, depth + 1);
                    }
                };

                (start_branch_depth + other_branch_depth, start_branch_depth + 1)
            };

            *result = (*result).max(candidate);

            (true, depth)
        })
    }

    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        let mut result = 0;

        Self::helper(root.as_deref(), start, &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        Self::amount_of_time(root, start)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
