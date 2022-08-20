use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        iter::successors(head.as_deref(), |node| node.next.as_deref()).fold(0, |acc, node| acc * 2 + node.val)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        Self::get_decimal_value(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
