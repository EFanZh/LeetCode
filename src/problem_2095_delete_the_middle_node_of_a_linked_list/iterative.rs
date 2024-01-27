use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::{iter, mem};

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let count = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let mut result = head;
        let mut node = &mut result;

        for _ in 0..count / 2 {
            node = &mut node.as_deref_mut().unwrap().next;
        }

        let rest = node.take().unwrap().next;

        drop(mem::replace(node, rest));

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::delete_middle(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
