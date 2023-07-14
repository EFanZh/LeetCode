use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::{iter, mem};

impl Solution {
    fn make_iter(mut head: Option<&mut ListNode>) -> impl Iterator<Item = &mut i32> {
        iter::from_fn(move || {
            head.take().map(|node| {
                let result = &mut node.val;

                head = node.next.as_deref_mut();

                result
            })
        })
    }

    pub fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let k = k as u32 as usize;
        let n = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let right = n - k;
        let left = k - 1;

        let (left, right) = match left.cmp(&right) {
            Ordering::Less => (left, right),
            Ordering::Equal => return head,
            Ordering::Greater => (right, left),
        };

        let mut iter = Self::make_iter(head.as_deref_mut());

        for _ in 0..left {
            iter.next();
        }

        let left_val = iter.next().unwrap();

        for _ in left + 1..right {
            iter.next();
        }

        mem::swap(left_val, iter.next().unwrap());

        drop(iter);

        head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn swap_nodes(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Self::swap_nodes(head, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
