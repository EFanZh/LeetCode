pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        let mut sorted_nums = nums.clone();

        sorted_nums.sort_unstable();

        let mut map = HashMap::new();
        let mut prev = -1;

        for (i, num) in (0..).zip(sorted_nums) {
            if num != prev {
                map.insert(num, i);
                prev = num;
            }
        }

        let mut result = nums;

        for num in &mut result {
            *num = map.get(num).copied().unwrap_or(0);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        Self::smaller_numbers_than_current(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
