use super::super::data_structures::TreeNode;

pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn get_left_height(mut root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;

        while let Some(node) = root {
            result += 1;

            root = node.borrow().left.clone();
        }

        result
    }

    fn count_nodes_helper(left_height: i32, node: &TreeNode) -> i32 {
        let right_height = Self::get_left_height(node.right.clone());

        if right_height == left_height {
            node.right.as_deref().map_or(1, |right| {
                (1 << left_height) + Self::count_nodes_helper(right_height - 1, &right.borrow())
            })
        } else {
            (1 << right_height) + Self::count_nodes_helper(right_height, &node.left.as_deref().unwrap().borrow())
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        root.map_or(0, |node| {
            let node = node.borrow();
            let left_height = Self::get_left_height(node.left.clone());

            Self::count_nodes_helper(left_height, &node)
        })
    }
}

impl super::Solution for Solution {
    fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::count_nodes(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
