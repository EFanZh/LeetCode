pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    fn reverse(mut num: u32) -> u32 {
        let mut reversed = 0;

        while num != 0 {
            reversed = reversed * 10 + num % 10;
            num /= 10;
        }

        reversed
    }

    pub fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        nums.iter()
            .flat_map(|&num| [num as _, Self::reverse(num as _)])
            .collect::<HashSet<_>>()
            .len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_distinct_integers(nums: Vec<i32>) -> i32 {
        Self::count_distinct_integers(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
