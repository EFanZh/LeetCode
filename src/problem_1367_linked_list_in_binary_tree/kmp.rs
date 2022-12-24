use crate::data_structures::{ListNode, TreeNode};

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn get_list_and_kmp_prefix_function(mut head: Option<Box<ListNode>>) -> Vec<(u8, u8)> {
        let mut result = Vec::<(u8, u8)>::new();
        let mut matched = 0;

        while let Some(node) = head {
            let c = node.val as _;

            if let Some(&(x, _)) = result.get(matched) {
                loop {
                    if c == x {
                        matched += 1;

                        break;
                    } else if let Some(&(_, next_matched)) = result.get(matched.wrapping_sub(1)) {
                        matched = usize::from(next_matched);
                    } else {
                        break;
                    }
                }
            }

            result.push((c, matched as _));
            head = node.next;
        }

        result
    }

    fn helper(node: Option<&RefCell<TreeNode>>, list_and_prefix_function: &[(u8, u8)], mut matched: usize) -> bool {
        if matched == list_and_prefix_function.len() {
            true
        } else if let Some(node) = node.map(RefCell::borrow) {
            let c = node.val as u8;

            loop {
                if c == list_and_prefix_function[matched].0 {
                    matched += 1;

                    break;
                } else if let Some(&(_, next_matched)) = list_and_prefix_function.get(matched.wrapping_sub(1)) {
                    matched = usize::from(next_matched);
                } else {
                    break;
                }
            }

            Self::helper(node.left.as_deref(), list_and_prefix_function, matched)
                || Self::helper(node.right.as_deref(), list_and_prefix_function, matched)
        } else {
            false
        }
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::helper(root.as_deref(), &Self::get_list_and_kmp_prefix_function(head), 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_sub_path(head, root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
