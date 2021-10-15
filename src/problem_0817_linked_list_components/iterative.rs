use crate::data_structures::ListNode;

pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;
use std::iter;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut in_nums = HashSet::with_capacity(nums.len());

        in_nums.extend(nums);

        let mut result = 0;
        let mut prev = false;

        for node in iter::successors(head.as_deref(), |node| node.next.as_deref()) {
            let current = in_nums.contains(&node.val);

            if !prev && current {
                result += 1;
            }

            prev = current;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        Self::num_components(head, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
