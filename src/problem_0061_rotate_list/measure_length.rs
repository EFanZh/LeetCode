use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let k = (k as usize).checked_rem(length).unwrap_or(0);

        let mut rest = {
            let mut maybe_node = &mut head;

            for _ in 0..(length - k) {
                maybe_node = &mut maybe_node.as_deref_mut().unwrap().next;
            }

            maybe_node.take()
        };

        let rest_tail = {
            let mut maybe_node = &mut rest;

            while let Some(node) = maybe_node {
                maybe_node = &mut node.next;
            }

            maybe_node
        };

        *rest_tail = head;

        rest
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Self::rotate_right(head, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
