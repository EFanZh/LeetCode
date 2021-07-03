use crate::data_structures::{ListNode, TreeNode};

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

impl Solution {
    fn sorted_list_to_bst_helper<I: Iterator<Item = i32>>(
        values: &mut I,
        length: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if length == 0 {
            None
        } else {
            let half = length / 2;
            let left = Self::sorted_list_to_bst_helper(values, half);
            let val = values.next().unwrap();
            let right = Self::sorted_list_to_bst_helper(values, length - half - 1);

            Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
        }
    }

    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let iterator = iter::successors(head.as_deref(), |node| node.next.as_deref());
        let length = iterator.clone().count();

        Self::sorted_list_to_bst_helper(&mut iterator.map(|node| node.val), length)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::sorted_list_to_bst(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
