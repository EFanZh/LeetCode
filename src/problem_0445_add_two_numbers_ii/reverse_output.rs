use super::super::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::{iter, mem};

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut length_1 = iter::successors(l1.as_deref(), |x| x.next.as_deref()).count();
        let mut length_2 = iter::successors(l2.as_deref(), |x| x.next.as_deref()).count();

        if length_2 < length_1 {
            mem::swap(&mut l1, &mut l2);
            mem::swap(&mut length_1, &mut length_2);
        }

        let mut reversed_result = None;
        let iter_1 = iter::successors(l1.as_deref(), |x| x.next.as_deref()).map(|x| x.val);
        let mut iter_2 = iter::successors(l2.as_deref(), |x| x.next.as_deref()).map(|x| x.val);

        for _ in 0..length_2 - length_1 {
            reversed_result = Some(Box::new(ListNode {
                val: iter_2.next().unwrap(),
                next: reversed_result,
            }));
        }

        for (lhs, rhs) in iter_1.zip(iter_2) {
            reversed_result = Some(Box::new(ListNode {
                val: lhs + rhs,
                next: reversed_result,
            }));
        }

        let mut result = None;
        let mut carry = 0;

        while let Some(mut node) = reversed_result {
            node.val += carry;

            if node.val > 9 {
                node.val -= 10;
                carry = 1;
            } else {
                carry = 0;
            }

            reversed_result = mem::replace(&mut node.next, result);
            result = Some(node);
        }

        if carry != 0 {
            result = Some(Box::new(ListNode { val: 1, next: result }));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::add_two_numbers(l1, l2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
