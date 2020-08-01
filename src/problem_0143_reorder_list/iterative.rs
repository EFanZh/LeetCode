use super::super::data_structures::ListNode;

pub struct Solution {}

use std::iter;
use std::mem;

impl Solution {
    pub fn reorder_list(mut head: &mut Option<Box<ListNode>>) {
        // Split at middle.

        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let half = (length + 1) / 2;

        let mut node = &mut *head;

        for _ in 0..half {
            node = &mut node.as_deref_mut().unwrap().next;
        }

        // Reverse list 2.

        let mut list_2 = None;
        let mut maybe_node = node.take();

        while let Some(mut node) = maybe_node {
            maybe_node = mem::replace(&mut node.next, list_2);
            list_2 = Some(node);
        }

        // Zip.

        let mut list_1 = head.take();

        while let Some(mut node) = list_1 {
            list_1 = node.next.take();
            head = &mut head.get_or_insert(node).next;

            if let Some(mut node) = list_2 {
                list_2 = node.next.take();
                head = &mut head.get_or_insert(node).next;
            } else {
                break;
            }
        }
    }
}

impl super::Solution for Solution {
    fn reorder_list(head: &mut Option<Box<ListNode>>) {
        Self::reorder_list(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
