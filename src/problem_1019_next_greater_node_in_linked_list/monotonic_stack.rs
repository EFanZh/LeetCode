use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;

impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let iter = iter::successors(head.as_deref(), |node| node.next.as_deref()).map(|node| node.val);
        let mut result = vec![0; iter.clone().count()];
        let mut stack = Vec::<(&mut i32, i32)>::new();

        for (target, value) in result.iter_mut().zip(iter) {
            while let Some((prev_target, prev_value)) = stack.last_mut() {
                if value > *prev_value {
                    **prev_target = value;

                    stack.pop();
                } else {
                    break;
                }
            }

            stack.push((target, value));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        Self::next_larger_nodes(head)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
