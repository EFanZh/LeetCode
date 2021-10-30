use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::mem;

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut head = head;
        let length = iter::successors(head.as_deref(), |node| node.next.as_deref()).count();
        let half = length / 2;
        let mut reversed = None;

        for _ in 0..half {
            let mut node = head.unwrap();

            head = mem::replace(&mut node.next, reversed);
            reversed = Some(node);
        }

        if length % 2 != 0 {
            head = head.unwrap().next;
        }

        reversed == head
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        Self::is_palindrome(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
