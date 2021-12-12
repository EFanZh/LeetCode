use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let dropped = length / 2;
        let mut node = head;

        for _ in 0..dropped {
            node = node.unwrap().next;
        }

        node
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::middle_node(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
