use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;

impl Solution {
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut nodes = Vec::<Box<ListNode>>::new();
        let mut sums = HashMap::from([(0, 0)]);
        let mut sum = 0;

        while let Some(mut node) = head {
            head = node.next.take();
            sum += node.val;

            match sums.entry(sum) {
                Entry::Occupied(entry) => {
                    let mut current_sum = sum - node.val;

                    for node in nodes.drain(*entry.get()..).rev() {
                        sums.remove(&current_sum);
                        current_sum -= node.val;
                    }
                }
                Entry::Vacant(entry) => {
                    nodes.push(node);
                    entry.insert(nodes.len());
                }
            }
        }

        let mut head = None;
        let mut slot = &mut head;

        for node in nodes {
            slot = &mut slot.insert(node).next;
        }

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::remove_zero_sum_sublists(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
