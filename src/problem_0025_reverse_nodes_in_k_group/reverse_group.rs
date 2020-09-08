use super::super::data_structures::ListNode;

pub struct Solution;

use std::iter;
use std::mem;

impl Solution {
    pub fn reverse_k_group(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut target = &mut result;
        let length = iter::successors(head.as_ref(), |node| node.next.as_ref()).count();

        for _ in 0..length / k as usize {
            // Build reversed group.

            let mut group = None;

            for _ in 0..k {
                let mut node = head.unwrap();

                head = mem::replace(&mut node.next, group);
                group = Some(node);
            }

            // Append the reversed group to the result chain.

            *target = group;

            // Find the next target.

            while let Some(node) = target {
                target = &mut node.next;
            }
        }

        // Append final group to the chain.

        *target = head;

        result
    }
}

impl super::Solution for Solution {
    fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Self::reverse_k_group(head, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
