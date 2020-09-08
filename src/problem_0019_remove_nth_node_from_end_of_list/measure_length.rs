use super::super::data_structures::ListNode;

pub struct Solution;

use std::iter;

impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let index_to_remove = length - (n as usize);

        let mut node = &mut head;

        for _ in 0..index_to_remove {
            node = &mut node.as_mut().unwrap().next;
        }

        *node = node.as_mut().unwrap().next.take();

        head
    }
}

impl super::Solution for Solution {
    fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::remove_nth_from_end(head, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
