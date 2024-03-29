use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Entry(Box<ListNode>);

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        other.0.val.eq(&self.0.val)
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.val.cmp(&self.0.val)
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut result = None;
        let mut target = &mut result;

        let mut heap = lists
            .into_iter()
            .filter_map(|list| list.map(Entry))
            .collect::<BinaryHeap<_>>();

        while let Some(list) = heap.pop() {
            target = &mut target.insert(list.0).next;

            if let Some(next) = target.take() {
                heap.push(Entry(next));
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        Self::merge_k_lists(lists)
    }
}

#[cfg(test)]
mod tests {
    use super::Entry;
    use crate::data_structures::ListNode;

    #[test]
    fn test_entry() {
        assert!(
            Entry(Box::new(ListNode { val: 2, next: None }))
                == Entry(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
