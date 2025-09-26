use crate::data_structures::TreeNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn count_nodes(node: Option<&RefCell<TreeNode>>) -> u32 {
        node.map_or(0, |node| {
            let node = node.borrow();

            1 + Self::count_nodes(node.left.as_deref()) + Self::count_nodes(node.right.as_deref())
        })
    }

    fn helper(node: Option<&RefCell<TreeNode>>, x: i32) -> Result<(u32, u32), u32> {
        node.map_or(Err(0), |node| {
            let node = node.borrow();

            if node.val == x {
                let left_count = Self::count_nodes(node.left.as_deref());
                let right_count = Self::count_nodes(node.right.as_deref());

                Ok((left_count, right_count))
            } else {
                Self::helper(node.left.as_deref(), x).or_else(|left_count| {
                    Self::helper(node.right.as_deref(), x).map_err(|right_count| 1 + left_count + right_count)
                })
            }
        })
    }

    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let n = n as u32;
        let (left_count, right_count) = Self::helper(root.as_deref(), x).unwrap();
        let rest = n - (1 + left_count + right_count);
        let max = left_count.max(right_count).max(rest);

        max > (n - max)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        Self::btree_game_winning_move(root, n, x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
