use super::super::data_structures::ListNode;

pub struct Solution {}

use std::iter;

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

                head = node.next.take();
                node.next = group;
                group = Some(node);
            }

            // Append the reversed group to the result chain.

            *target = group;

            // Find the next target.

            let mut node = target;

            while let Some(node_2) = node {
                node = &mut node_2.next;
            }

            target = node;
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
    use super::super::tests::run_tests;
    use super::Solution;

    #[test]
    fn test_solution() {
        run_tests::<Solution>();
    }
}
