use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::mem;

impl Solution {
    fn make_iter(list: Option<&ListNode>) -> impl Iterator<Item = &ListNode> {
        iter::successors(list, |node| node.next.as_deref())
    }

    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut head = head;
        let length = Self::make_iter(head.as_deref()).count();
        let mut node = &mut head;

        for _ in 0..length / 2 {
            node = &mut node.as_deref_mut().unwrap().next;
        }

        let mut right = None;
        let mut maybe_node = node.take();

        while let Some(mut node) = maybe_node {
            maybe_node = mem::replace(&mut node.next, right);
            right = Some(node);
        }

        Self::make_iter(head.as_deref())
            .zip(Self::make_iter(right.as_deref()))
            .map(|(left, right)| left.val + right.val)
            .max()
            .unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        Self::pair_sum(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
