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

        let mut iter =
            iter::successors(head.as_deref(), |node| node.next.as_deref()).map(|node| in_nums.contains(&node.val));

        'outer: while let Some(current) = iter.next() {
            // Previous value is not in `nums`.

            if current {
                result += 1;

                loop {
                    if let Some(current) = iter.next() {
                        // Previous value is in `nums`.

                        if !current {
                            break;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
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
