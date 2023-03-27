pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        let mut iter = nums.iter().copied();
        let mut set = HashSet::new();
        let mut result = 0;

        'outer: while let Some(mut sum) = iter.next() {
            set.insert(0);

            while !set.contains(&(sum - target)) {
                set.insert(sum);

                if let Some(num) = iter.next() {
                    sum += num;
                } else {
                    break 'outer;
                }
            }

            result += 1;
            set.clear();
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_non_overlapping(nums: Vec<i32>, target: i32) -> i32 {
        Self::max_non_overlapping(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
