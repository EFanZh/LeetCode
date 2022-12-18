use crate::data_structures::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub mod iterator;
pub mod rebuild_from_scratch;

pub trait Solution {
    fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::data_structures::TreeNode;
    use crate::test_utilities;
    use std::cell::RefCell;

    fn check_balance(node: Option<&RefCell<TreeNode>>) -> u32 {
        node.map_or(0, |node| {
            let node = node.borrow();

            let left_depth = check_balance(node.left.as_deref());
            let right_depth = check_balance(node.right.as_deref());

            assert!(left_depth.abs_diff(right_depth) <= 1);

            left_depth.max(right_depth) + 1
        })
    }

    pub fn run<S: Solution>() {
        let test_cases = [
            &[Some(1), None, Some(2), None, Some(3), None, Some(4), None, None] as &[_],
            &[Some(2), Some(1), Some(3)],
        ];

        for root in test_cases {
            let root = test_utilities::make_tree(root.iter().copied());
            let values = test_utilities::iter_tree_in_order(root.clone()).collect::<Vec<_>>();
            let result = S::balance_bst(root);

            check_balance(result.as_deref());

            assert!(values
                .iter()
                .copied()
                .eq(test_utilities::iter_tree_in_order(result.clone())));
        }
    }
}
