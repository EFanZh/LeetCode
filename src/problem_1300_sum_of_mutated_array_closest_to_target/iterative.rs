pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        let mut queue = arr.into_iter().map(Reverse).collect::<BinaryHeap<_>>();
        let mut required = target;
        let mut remaining = queue.len() as i32;
        let mut prev = 0;

        while let Some(Reverse(value)) = queue.pop() {
            if value * remaining >= required {
                let candidate = required / remaining;

                return if (candidate * 2 + 1) * remaining < required * 2 {
                    candidate + 1
                } else {
                    candidate
                };
            }

            required -= value;
            remaining -= 1;
            prev = value;
        }

        prev
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_best_value(arr: Vec<i32>, target: i32) -> i32 {
        Self::find_best_value(arr, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
