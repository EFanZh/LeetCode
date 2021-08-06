use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let k = k as usize;
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let min_length = length / k;
        let max_length_count = length % k;
        let max_length = min_length + 1;
        let min_length_count = k - max_length_count;
        let mut result = Vec::with_capacity(k);
        let mut node = head;

        for &(length, count) in &[(max_length, max_length_count), (min_length, min_length_count)] {
            for _ in 0..count {
                let mut sub_list = None;
                let mut tail = &mut sub_list;

                for _ in 0..length {
                    let mut current = node.unwrap();

                    node = current.next.take();

                    tail = &mut tail.get_or_insert(current).next;
                }

                result.push(sub_list);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        Self::split_list_to_parts(head, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
