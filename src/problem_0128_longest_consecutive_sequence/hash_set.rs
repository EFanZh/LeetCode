pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().collect::<HashSet<_>>();
        let mut result = 0;

        for num in nums.iter().copied() {
            if !nums.contains(&(num - 1)) {
                let mut end = num + 1;

                while nums.contains(&end) {
                    end += 1;
                }

                result = result.max(end - num);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_consecutive(nums: Vec<i32>) -> i32 {
        Self::longest_consecutive(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
